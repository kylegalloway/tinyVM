use std::fs::File;
///
/// |=============================== INSTRUCTION ================================|
///  COMMAND                VALUE             DESCRself.ipTION
/// -------------------------------------------------------------------------------
///  HLT                      0        :: halts the program
///
///  PSH, [value ]            1        :: pushes <value> to stack
///  POP                      2        :: pops <value> from stack
///
///  ADD                      3        :: adds top two values on stack
///  MUL                      4        :: multiplies top two values on stack
///  DIV                      5        :: divides top two values on stack
///  SUB                      6        :: subtracts top two values on stack
///
///  SLT, [reg_a], [reg_b]    7        :: pushes (reg_a < reg_b) to stack
///
///  MOV, [reg_a], [reg_b]    8        :: moves the value in reg_a to reg_b
///  SET, [reg], [value]      9        :: sets the reg to <value>
///
///  LOG, [reg]               10       :: prints out the value in reg
///
///  IF, [reg] [value] [IP]   11       :: if (register == value) branch to IP
///  IFN, [reg] [value] [IP]  12       :: if (register != value) branch to IP
///
///  GLD, [reg]               13       :: loads a register to the stack
///  GPT, [reg]               14       :: pushes top of stack to the given register
///
///  GOTO, [IP]               15       :: goes to the instruction at IP
///  NOP                      16       :: do nothing
///

use std::io;
use std::io::BufReader;
use std::io::prelude::*;

pub fn main(file: String)
{
    let stack_size = 256;
    let register_size = 13;
    let instruction_count = 10;
    let instruction_space = 4;

    let f = File::open(file).expect("Bad File");
    let reader = BufReader::new(f);
    let mut program: Vec<u32> = Vec::new();

    for line in reader.lines()
    {
        let mut new: Vec<u32> = line.expect("Bad line")
            .split(" ")
            .map(|x| x.parse::<u32>().expect("Not an integer!"))
            .collect();
        program.extend(new);
    }

    let mut vm = Vm::new(stack_size, register_size, instruction_count, instruction_space);


    while vm.is_running
    {
        let instruction: Instruction = Instruction::from(vm.fetch(&program));
        vm.eval(instruction, &program);
        vm.cycle();
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction
{
    HLT,
    PSH,
    POP,
    ADD,
    MUL,
    DIV,
    SUB,
    SLT,
    MOV,
    SET,
    LOG,
    IF,
    IFN,
    GLD,
    GPT,
    GOTO,
    NOP,
}

impl From<u32> for Instruction
{
    fn from(i: u32) -> Instruction
    {
        match i
        {
            0 => Instruction::HLT,
            1 => Instruction::PSH,
            2 => Instruction::POP,
            3 => Instruction::ADD,
            4 => Instruction::MUL,
            5 => Instruction::DIV,
            6 => Instruction::SUB,
            7 => Instruction::SLT,
            8 => Instruction::MOV,
            9 => Instruction::SET,
            10 => Instruction::LOG,
            11 => Instruction::IF,
            12 => Instruction::IFN,
            13 => Instruction::GLD,
            14 => Instruction::GPT,
            15 => Instruction::GOTO,
            16 | _ => Instruction::NOP,
        }
    }
}

#[derive(Debug)]
enum Registers
{
    A,
    B,
    C,
    D,
    E,
    F,
    I,
    J,
    EX,
    EXA,
    IP,
    SP,
}

#[derive(Debug)]
struct Vm
{
    is_running: bool,
    ip: u32,
    sp: i32,
    stack: Vec<u32>,
    registers: Vec<u32>,
    instruction_count: u32,
    instruction_space: u32,
}

impl Vm
{
    fn new(stack_size: usize,
           register_size: usize,
           instruction_count: u32,
           instruction_space: u32)
           -> Vm
    {
        Vm {
            is_running: true,
            ip: 0,
            sp: -1,
            stack: vec![0u32; stack_size],
            registers: vec![0u32; register_size],
            instruction_count: instruction_count,
            instruction_space: instruction_space,
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

    fn print_stack(&mut self)
    {
        println!("Stack:\n{:?}\n", self.stack);
    }

    fn print_registers(&mut self)
    {
        println!("Registers:\n{:?}\n", self.registers);
    }

    fn find_empty_register(&mut self) -> Registers
    {
        return Registers::EX;
    }

    fn eval(&mut self, instruction: Instruction, program: &Vec<u32>)
    {
        use Instruction::*;
        use Registers::*;
        match instruction
        {
            HLT =>
            {
                self.is_running = false;
                println!("Finished Execution\n");
                // self.print_stack();
                // self.print_registers();

            },
            PSH =>
            {
                self.sp = self.sp + 1;
                self.ip = self.ip + 1;
                self.stack[self.registers[SP as usize] as usize] = program[self.registers[IP as usize] as
                usize];
                println!("{} pushed to stack[{}]\n",
                         self.stack[self.registers[SP as usize] as usize],
                         self.sp);

            },
            POP =>
            {
                println!("{} popped from stack[{}]\n",
                         self.stack[self.registers[SP as usize] as usize],
                         self.sp);
                self.sp = self.sp - 1;

            },
            ADD =>
            {
                self.registers[A as usize] = self.stack[self.registers[SP as usize] as usize];
                self.sp = self.sp - 1;

                self.registers[B as usize] = self.stack[self.registers[SP as usize] as usize];
                /* self.sp = self.sp - 1; */

                self.registers[C as usize] = self.registers[B as usize] +
                                             self.registers[A as usize];

                /* self.sp = self.sp + 1; */
                self.stack[self.registers[SP as usize] as usize] = self.registers[C as usize];
                println!("{} + {} = {}\n",
                         self.registers[B as usize],
                         self.registers[A as usize],
                         self.registers[C as usize]);
                println!("{} moved to stack[{}]\n",
                         self.stack[self.registers[SP as usize] as usize],
                         self.sp);

            },
            MUL =>
            {
                self.registers[A as usize] = self.stack[self.registers[SP as usize] as usize];
                self.sp = self.sp - 1;

                self.registers[B as usize] = self.stack[self.registers[SP as usize] as usize];
                /*self.sp = self.sp - 1;*/

                self.registers[C as usize] = self.registers[B as usize] *
                                             self.registers[A as usize];

                /*self.sp = self.sp + 1;*/
                self.stack[self.registers[SP as usize] as usize] = self.registers[C as usize];
                println!("{} * {} = {}\n",
                         self.registers[B as usize],
                         self.registers[A as usize],
                         self.registers[C as usize]);
                println!("{} moved to stack[{}]\n",
                         self.stack[self.registers[SP as usize] as usize],
                         self.sp);

            },
            DIV =>
            {
                self.registers[A as usize] = self.stack[self.registers[SP as usize] as usize];
                self.sp = self.sp - 1;

                self.registers[B as usize] = self.stack[self.registers[SP as usize] as usize];
                /* self.sp = self.sp - 1;*/

                self.registers[C as usize] = self.registers[B as usize] /
                                             self.registers[A as usize];

                /* self.sp = self.sp + 1; */
                self.stack[self.registers[SP as usize] as usize] = self.registers[C as usize];
                println!("{} / {} = {}\n",
                         self.registers[B as usize],
                         self.registers[A as usize],
                         self.registers[C as usize]);
                println!("{} moved to stack[{}]\n",
                         self.stack[self.registers[SP as usize] as usize],
                         self.sp);

            },
            SUB =>
            {
                self.registers[A as usize] = self.stack[self.registers[SP as usize] as usize];
                self.sp = self.sp - 1;

                self.registers[B as usize] = self.stack[self.registers[SP as usize] as usize];
                /* self.sp = self.sp - 1; */

                self.registers[C as usize] = self.registers[B as usize] -
                                             self.registers[A as usize];

                /* self.sp = self.sp + 1; */
                self.stack[self.registers[SP as usize] as usize] = self.registers[C as usize];
                println!("{} - {} = {}\n",
                         self.registers[B as usize],
                         self.registers[A as usize],
                         self.registers[C as usize]);
                println!("{} moved to stack[{}]\n",
                         self.stack[self.registers[SP as usize] as usize],
                         self.sp);

            },
            SLT =>
            {
                self.sp = self.sp - 1;
                self.stack[self.registers[SP as usize] as usize] =
                    (self.stack[(self.sp + 1) as usize] <
                     self.stack[self.registers[SP as usize] as usize]) as u32;

            },
            MOV =>
            {
                self.registers[program[(self.ip + 2) as usize] as usize] = self.registers[program[(self.ip + 1) as usize] as
                usize];
                self.ip = self.ip + 2;

            },
            SET =>
            {
                self.registers[program[(self.ip + 1) as usize] as usize] = program[(self.ip + 2) as
                usize];
                self.ip = self.ip + 2;

            },
            LOG =>
            {
                println!("{}\n", self.registers[program[(self.ip + 1) as usize] as usize]);
                self.ip = self.ip + 1;

            },
            IF =>
            {
                if self.registers[program[(self.ip + 1) as usize] as usize] ==
                   program[(self.ip + 2) as usize]
                {
                    self.registers[(self.ip + 1) as usize] = program[(self.ip + 3) as usize];
                }
                self.ip = self.ip + 3;

            },
            IFN =>
            {
                if self.registers[program[(self.ip + 1) as usize] as usize] !=
                   program[(self.ip + 2) as usize]
                {
                    self.registers[EX as usize] = program[(self.ip + 3) as usize];
                    let ip_: usize = self.registers[IP as usize] as usize;
                    self.registers[ip_] = self.registers[EX as usize];
                }
                else
                {
                    self.ip = self.ip + 3;
                }
            },
            GLD =>
            {
                self.sp = self.sp + 1;
                self.ip = self.ip + 1;
                self.stack[self.registers[SP as usize] as usize] = self.registers[program[self.registers[IP as usize] as usize] as
                usize];

            },
            GPT =>
            {
                self.registers[program[(self.ip + 1) as usize] as usize] = self.stack[self.registers[SP as usize] as
                usize];
                println!("{} loaded into stack[{}]\n",
                         self.stack[self.registers[SP as usize] as usize],
                         self.sp);
                self.ip = self.ip + 1;

            },
            GOTO =>
            {
                if program[(self.ip + 1) as usize] > 0 &&
                   program[(self.ip + 1) as usize] < self.instruction_count
                {
                    self.ip = program[(self.ip + 1) as usize];
                    println!("branch to instruction {}\n", program[(self.ip + 1) as usize]);
                }
                else
                {
                    println!("branch to instruction {} not completed.\n",
                             program[(self.ip + 1) as usize]);
                    println!("{} not not i range.\n", program[(self.ip + 1) as usize]);
                }
            },
            NOP =>
            {
                println!("Do Nothing\n");

            },
        }
    }
}