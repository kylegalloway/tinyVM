pub fn main()
{
    let mut vm = Vm::new();

    let program: Vec<u32> = vec![InstructionSet::PSH as u32,
                                 5,
                                 InstructionSet::PSH as u32,
                                 6,
                                 InstructionSet::ADD as u32,
                                 InstructionSet::POP as u32,
                                 InstructionSet::HLT as u32];

    while vm.is_running
    {
        let instruction = match vm.fetch(&program)
        {
            0 => InstructionSet::PSH,
            1 => InstructionSet::ADD,
            2 => InstructionSet::POP,
            3 | _ => InstructionSet::HLT,
        };
        vm.eval(instruction, &program);
        vm.cycle();
    }
}

#[derive(Debug, Eq, PartialEq)]
enum InstructionSet
{
    PSH,
    ADD,
    POP,
    HLT,
}

#[derive(Debug)]
struct Vm
{
    is_running: bool,
    ip: u32,
    sp: i32,
    stack: Vec<u32>,
}

impl Vm
{
    fn new() -> Vm
    {
        Vm {
            is_running: true,
            ip: 0,
            sp: -1,
            stack: vec![0u32; 256],
        }
    }

    fn cycle(&mut self)
    {
        self.ip += 1;
    }

    fn fetch(&mut self, program: &Vec<u32>) -> u32
    {
        program[self.ip as usize]
    }

    fn eval(&mut self, instruction: InstructionSet, program: &Vec<u32>)
    {
        use InstructionSet::*;
        match instruction
        {
            PSH =>
            {
                self.sp += 1;
                self.ip += 1;
                self.stack[self.sp as usize] = program[self.ip as usize];
            },
            ADD =>
            {
                let a = self.stack[self.sp as usize];
                self.sp -= 1;
                let b = self.stack[self.sp as usize];
                self.sp -= 1;
                let result = b + a;
                self.sp += 1;
                self.stack[self.sp as usize] = result;
            },
            POP =>
            {
                let val_popped = self.stack[self.sp as usize];
                self.sp -= 1;
                println!("popped {}\n", val_popped);
            },
            HLT =>
            {
                self.is_running = false;
                println!("done\n");
            },
        }
    }
}