const std=@import("std");const c=@import("common.zig");
fn run(nodes:[]const c.Node,res:[]i64)void{var i:usize=0;while(i<nodes.len):(i+=1){const n=nodes[i];switch(n.op){0=>res[i]=@intCast(n.a),1=>res[i]=c.heavy(1,res[n.a],res[n.b]),2=>res[i]=c.heavy(1,res[n.b],res[n.a]),else=>return,}}}
pub fn main()void{const a=std.heap.page_allocator;c.report("switch",1,c.gen(a),a.alloc(i64,c.N) catch unreachable,&run);}
