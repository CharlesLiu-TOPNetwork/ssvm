# 写一个简单的堆栈虚拟机: ssvm
**simple stack-based virtual machine.**

需要rust基础语法，能看懂代码。
相关语言知识教程：英文版推荐官方[book](https://doc.rust-lang.org/stable/book/title-page.html)，中文版推荐中文社区的[Rust语言圣经](https://course.rs/basic/intro.html)，了解语法的基础章节即可，至少包括以下内容（相关语言关键字）：
* 变量`let`
* 基础数值、字符类型
* 类型对象`struct`
* 枚举对象`enum`
* 实现类型对象的方法`impl ...`
* 模式匹配`match`


## 仓库链接：
https://github.com/CharlesLiu-TOPNetwork/ssvm/tree/demo/literal_instruction

## 虚拟机基础结构介绍
![ssvm_structs](./%40resources/ssvm_structs.png)

### stack
* 后进先出
* 深度限制
* 栈内元素
    * 定长(byte 位宽)
    * 不定长(literal element)，本ssvm选择该表达方式，作为demo可读性较强
* `push`入栈，`pop`出栈

### instructions && pc
instructions:
* 若干指令按照一定顺序形成的合集

pc:
* Program Counter：程序计数器，指向当前的指令，一般除了一些跳转指令外，其它指令执行后，pc自增1

### memory/storage
当前ssvm的demo阶段尽可能简单的实现内存`memory`的读写。还不涉及操作持久化的存储`storage`

## 步骤

1. 实现`Literal`操作数、支持整型和字符串
2. 实现`Machine`对象
3. 增加`Push`、`Display`、`Add`指令并实现相应功能
4. 增加支持其它运算指令`Sub`、`Mul`、`Div`
5. 增加支持浮点数f32
6. 增加内存模块，新指令`Mstore`、`Mload`

### 1. 实现`Literal`操作数、支持整型和字符串

``` RUST
#[derive(Debug, Clone, Copy)]
enum Literal {
    Int(i32),
    Str(&'static str),
}

fn main() {}

```


### 2. 实现`Machine`对象
``` RUST
#[derive(Debug, Clone, Copy)]
enum Instruction {}

struct Machine {
    stack: Vec<Literal>,
    code: Vec<Instruction>,
    pc: usize,
}

impl Machine {
    fn new(code: Vec<Instruction>) -> Machine {
        Machine {
            stack: Vec::new(),
            code,
            pc: 0,
        }
    }

    fn execute(&mut self) {
        while self.pc < self.code.len() {
            let opcode = self.code[self.pc];
            self.pc += 1;
            self.dispatch(opcode);
        }
    }

    fn dispatch(&mut self, opcode: Instruction) {
        match opcode {
            _ => todo!(),
        };
    }
}
```

### 3. 增加`Push`、`Display`、`Add`指令并实现相应功能
增加指令：
``` RUST
enum Instruction {
    Add,
    Push(Literal),
    Display,
}
```

对应的`push`/`pop`方法，和`dispatch`的具体对应实现：
``` RUST
impl Machine {
    // ...    
    fn push(&mut self, x: Literal) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> Literal {
        self.stack.pop().expect("stack popped while empty")
    }
    // ...

    fn dispatch(&mut self, opcode: Instruction) {
        match opcode {
            Instruction::Push(lit) => self.push(lit),
            Instruction::Add => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x + y),
                    _ => panic!("error in code-- expected int for binary operations"),
                });
            }

            Instruction::Display => {
                match self.pop() {
                    Literal::Int(x) => print!("{:?}", x),
                    Literal::Str(x) => print!("{}", x),
                }
                io::stdout().flush().unwrap();
            }
        };
    }
}
```

可以测试了：
``` RUST
fn main() {
    let mut machine = Machine::new(vec![
        Instruction::Push(Literal::Int(2)),
        Instruction::Push(Literal::Int(1)),
        Instruction::Add,
        Instruction::Push(Literal::Str("1 + 2 is ")),
        Instruction::Display,
        Instruction::Display,
        Instruction::Push(Literal::Str("\n")),
        Instruction::Display,
    ]);
    println!("------------ machine output ------------");
    machine.execute();
}
```

### 4. 增加支持其它运算指令`Sub`、`Mul`、`Div`
``` RUST
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Push(Literal),
    Display,
}
```

``` RUST
fn dispatch(&mut self, opcode: Instruction) {
        match opcode {
            // ...
            Instruction::Sub => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x - y),
                    _ => panic!("error in code-- expected int for binary operations"),
                });
            }
            Instruction::Mul => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x * y),
                    _ => panic!("error in code-- expected int for binary operations"),
                });
            }
            Instruction::Div => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x / y),
                    _ => panic!("error in code-- expected int for binary operations"),
                });
            }
        };
    }
```


### 5. 增加支持浮点数`f32`
``` RUST
#[derive(Debug, Clone, Copy)]
enum Literal {
    Int(i32),
    Float(f32),
    Str(&'static str),
}
```

修改对应的指令的实现：略

### 6. 增加内存模块，新指令`Mstore`、`Mload`
新增两个指令：
``` RUST
#[derive(Debug, Clone, Copy)]
enum Instruction {
    // ...
    Mstore((u32)),
    Mload((u32)),
}
```

新增machine成员变量：
``` RUST
struct Machine {
    // ...
    mem: HashMap<u32, Literal>,
}
```

新增对应的处理方法和指令实现：
``` RUST
impl Machine {
    fn mem_store(&mut self, addr: u32, value: Literal) {
        self.mem.insert(addr, value);
    }

    fn mem_load(&mut self, addr: &u32) -> Option<&Literal> {
        self.mem.get(addr)
    }

    fn dispatch(&mut self, opcode: Instruction) {
        match opcode {
            // ...
            Instruction::Mstore(addr) => {
                let value = self.pop();
                self.mem_store(addr, value);
            }
            Instruction::Mload(addr) => {
                let value = self.mem_load(&addr).unwrap().clone(); // todo error in code;
                self.push(value);
            }
        };
    }
}
```

## TODO
- [ ] 将literal的操作数转换为定长的字节码形式、如何编码？
- [ ] 新增方法上下文处理、CALL指令
- [ ] 增加错误处理机制
- [ ] 增加测试套件