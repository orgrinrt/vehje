const std=@import("std");const c=@import("common.zig");
const H=*const fn(usize,[*]const c.Node,[*]i64,usize)callconv(.c)void;var tbl:[4]H=undefined;
inline fn nx(i:usize,n:[*]const c.Node,r:[*]i64,l:usize)void{const j=i+1;if(j>=l)return;@call(.always_tail,tbl[n[j].op],.{j,n,r,l});}
fn hl(i:usize,n:[*]const c.Node,r:[*]i64,l:usize)callconv(.c)void{r[i]=@intCast(n[i].a);nx(i,n,r,l);}
fn ha(i:usize,n:[*]const c.Node,r:[*]i64,l:usize)callconv(.c)void{r[i]=c.heavy(4,r[n[i].a],r[n[i].b]);nx(i,n,r,l);}
fn hm(i:usize,n:[*]const c.Node,r:[*]i64,l:usize)callconv(.c)void{r[i]=c.heavy(4,r[n[i].b],r[n[i].a]);nx(i,n,r,l);}
fn hh(i:usize,n:[*]const c.Node,r:[*]i64,l:usize)callconv(.c)void{_=i;_=n;_=r;_=l;}
fn run(nodes:[]const c.Node,res:[]i64)void{tbl=.{&hl,&ha,&hm,&hh};@call(.auto,tbl[nodes[0].op],.{@as(usize,0),nodes.ptr,res.ptr,nodes.len});}
pub fn main()void{const a=std.heap.page_allocator;c.report("tail",4,c.gen(a),a.alloc(i64,c.N) catch unreachable,&run);}
