import ast, sys, os, glob, collections
stdlib = os.path.dirname(os.__file__)
files = glob.glob(os.path.join(stdlib, "*.py")) + glob.glob(os.path.join(stdlib, "**/*.py"), recursive=True)
files = files[:1500]
arity = collections.Counter()   # call arg counts
depth_hist = collections.Counter()  # function nesting depth (live-binder-scope proxy)
scopes_live = collections.Counter() # locals per function (live-binder width proxy)
calls=0; funcs=0; closures=0; total_funcs=0
class V(ast.NodeVisitor):
    def __init__(self): self.depth=0
    def visit_Call(self, n):
        global calls
        a=len(n.args)+len(n.keywords); arity[min(a,8)]+=1; calls+=1; self.generic_visit(n)
    def visit_FunctionDef(self, n):
        global funcs,total_funcs,closures
        funcs+=1; total_funcs+=1; self.depth+=1; depth_hist[min(self.depth,8)]+=1
        # locals as a live-binder-width proxy
        names=set()
        for x in ast.walk(n):
            if isinstance(x,(ast.arg,)): names.add(x.arg)
            if isinstance(x,ast.Name) and isinstance(x.ctx,ast.Store): names.add(x.id)
        scopes_live[min(len(names)//8*8,64)]+=1
        # closure detection: an inner def referencing an outer local (avoidance proxy)
        if self.depth>1: closures+=1
        self.generic_visit(n); self.depth-=1
    visit_AsyncFunctionDef=visit_FunctionDef
for f in files:
    try: V().visit(ast.parse(open(f,encoding='utf-8',errors='ignore').read()))
    except Exception: pass
print(f"corpus: {len(files)} python files (stdlib proxy)")
tot=sum(arity.values())
print("CALL ARITY distribution (args+kwargs):")
for k in sorted(arity): print(f"  arity {k}{'+' if k==8 else ' '}: {100*arity[k]/tot:5.1f}%  ({arity[k]})")
print(f"  => arity<=2: {100*(arity[0]+arity[1]+arity[2])/tot:.1f}%   arity>=3: {100*sum(arity[k] for k in arity if k>=3)/tot:.1f}%")
print("FUNCTION NESTING DEPTH (live-scope-chain proxy):")
td=sum(depth_hist.values())
for k in sorted(depth_hist): print(f"  depth {k}{'+' if k==8 else ' '}: {100*depth_hist[k]/td:5.1f}%")
print("LIVE-BINDER WIDTH per function (locals, bucketed):")
ts=sum(scopes_live.values())
for k in sorted(scopes_live): print(f"  {k}-{k+7} binders: {100*scopes_live[k]/ts:5.1f}%")
print(f"CLOSURE rate (nested-def, avoidance-relevant): {100*closures/max(total_funcs,1):.1f}% of functions are nested")
