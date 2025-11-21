
# 🌌 **Singularity**

> **Bushcrafted DI for Rust**
> Handmade using traits, generics, and unreasonable optimism.
> So minimal even cave people can grow bonsai with it.

---

```
                      .+&@▒#*░█░#&-                 
                    .*▓████████████@=.                
                  :░██████████████████*             
                  =▓███▓▓=▓+░&@▒%█████#.            
                   █▒▓▒&▓▒░#-+@░▓▓&█▒█#             
                   +#@▒ =%++.:**=::█*@:             
                    %██&..@%-=&% :▒█░:              
                   :*███@█▓░#░░█▒▒██▓▒░%:             
            :+=::.:=-:*▓██████████░&@==*..:-=-      
           ++          .*@▓████▓#-.%▒*#.     -*        
           *+=+=.**             +#:   :%&..++-%:    
          =+ .   #█@-&=   :=: =*+#*.▒▓-#@:    :%    
          *-+   .▓█#+**%%+: .==..░░.=#▒██+   =.%    
         .%:=:.:%#▓:=.  .:--&=&░+..:+-▒██&:  :.%:                     
         .%   :=@░█▓*::▓▒ -█▓:   -&%.@██░=+-.  *=   
         *::.  @@██##@-::+=  :░▓*-#&.@██░#&= .-.%   
      :=&░▒%%=*%░██#+*-=.%*:*-*%-=*:#▓██@-#++++%=   
      =*░#:=+-:.=▓▒█▓░▓█▓#++░#=&▓█▓█@▒▒█% .:-+=.    
       ..        .=▒.=:+▓-.*#&-+█%.: #::            
                  .░*.+=%      .░▓=.&@              
                  =**#@**       #+%%*%+             
                  *%+.= @       @ = * %              
                   *@ @*         %- *+              
                   %% #%        .&- **              
                 .=+  .░        -@.  =+.            
         -+*%@@@##**=&-*░░░░░░░░░%-&++*&&@@@#%**:   
         ..-===*%%%%%%%%%%%%%%%%%%%%%%%%%%+===-..   
```


## 🌿 What is this?

**Singularity** is an experimental *compile-time* dependency injection concept for Rust.

🪵 Built using only:

* 🧠 Trait-based resolution
* 🔗 Generic type inference
* 🔥 No reflection
* ❌ No runtime container
* 🧊 No heap allocations from container logic

💡 Dependencies are resolved *recursively* by calling `container.Resolve::<MyService>()` — the container itself carries no state.

---

### 🪚 Example (carved with a rock)

```rust
use singularity::container::*;

#[derive(Debug)]
struct A(i32);
impl Injectable for A {
    type Deps = ();
    fn inject(_: ()) -> Self { A(10) }
}

#[derive(Debug)]
struct B(i32);
impl Injectable for B {
    type Deps = ();
    fn inject(_: ()) -> Self { B(32) }
}

struct ComputeSum;
impl Invokable for ComputeSum {
    type Deps = (A, B);
    type Output = i32;

    fn invoke_with<F>((a, b): (A, B), callback: F)
    where
        F: FnOnce(i32),
    {
        let result = {
            let sum = a.0 + b.0;
            sum // a,b burada öldü
        };

        println!("ComputeSum invoked internally, result = {result}");


    }
}

```

🪵 **Console output**

```
ComputeSum invoked internally, result = 42
```

---

## 📐 Core Philosophy

| Principle                  | Translation                                     |
| -------------------------- | ----------------------------------------------- |
| Compile-time DI            | “Why wait for runtime when type system can suffer?” |
| No container state         | Container is a mood, not a storage              |
| No service registry        | Services aren't *stored*, they're *manifested*  |
| No heap allocation         | "Heap? I live in cave."                         |
| Factory-based construction | behaves like a constructor — with compile-time dependency wiring |

---

## 📦 Current Status

🚧 Minimal concept — **not production-ready**
📌 Only supports:

* `Injectable` trait
* Recursive resolution
* Stateless container
* Compile-time validated dependencies

❌ Not implemented *(yet)*:

* Lifetime management
* Singleton/scoped services
* Handler systems
* Ordering/grouping
* Macros

---

## 🎴 Final Haiku

```
Trait calls trait again  
Stack of types whisper resolve  
Injection occurs
```


## 🪓 Closing Words

> *“If your DI framework needs reflection, let it survive in the wilderness without heap allocation.”*

## License **[MIT](./LICENSE)**