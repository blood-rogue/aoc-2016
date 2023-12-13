use itertools::Itertools;

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn parse(s: &str) -> Self {
        match s {
            "a" => Self::A,
            "b" => Self::B,
            "c" => Self::C,
            "d" => Self::D,
            _ => unreachable!(),
        }
    }

    const fn idx(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy)]
enum Operand {
    Num(isize),
    Register(Register),
}

#[derive(Clone, Copy)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Register),
    Dec(Register),
    Jnz(Operand, Operand),
    Tgl(Register),
}

fn main() {
    let mut registers = [7, 0, 0, 0];
    let mut instructions = include_str!(r"..\..\input\day23.txt")
        .lines()
        .map(|line| {
            let (instruction, operand) = line.split_once(' ').unwrap();

            match instruction {
                "cpy" => {
                    let (src, dst) = operand.split_once(' ').unwrap();

                    Instruction::Cpy(
                        src.parse::<isize>()
                            .map_or_else(|_| Operand::Register(Register::parse(src)), Operand::Num),
                        Operand::Register(Register::parse(dst)),
                    )
                }
                "inc" => Instruction::Inc(Register::parse(operand)),
                "dec" => Instruction::Dec(Register::parse(operand)),
                "jnz" => {
                    let (condition, offset) = operand.split_once(' ').unwrap();
                    Instruction::Jnz(
                        condition.parse::<isize>().map_or_else(
                            |_| Operand::Register(Register::parse(condition)),
                            Operand::Num,
                        ),
                        offset.parse::<isize>().map_or_else(
                            |_| Operand::Register(Register::parse(offset)),
                            Operand::Num,
                        ),
                    )
                }
                "tgl" => Instruction::Tgl(Register::parse(operand)),
                _ => unreachable!(),
            }
        })
        .collect_vec();

    let mut index = 0;

    while let Some(instruction) = instructions.get(index) {
        match instruction {
            Instruction::Cpy(src, dst) => {
                let src = match src {
                    Operand::Num(value) => *value,
                    Operand::Register(reg) => registers[reg.idx()],
                };

                if let Operand::Register(dst) = dst {
                    registers[dst.idx()] = src;
                };

                index += 1;
            }

            Instruction::Dec(operand) => {
                registers[operand.idx()] -= 1;
                index += 1;
            }

            Instruction::Inc(operand) => {
                registers[operand.idx()] += 1;
                index += 1;
            }

            Instruction::Jnz(cond, offset) => {
                let cond = match cond {
                    Operand::Num(value) => *value != 0,
                    Operand::Register(reg) => registers[reg.idx()] != 0,
                };

                let offset = match offset {
                    Operand::Num(value) => *value,
                    Operand::Register(reg) => registers[reg.idx()],
                };

                if cond {
                    index = index.checked_add_signed(offset).unwrap();
                } else {
                    index += 1;
                }
            }

            Instruction::Tgl(reg) => {
                let toggled_index = index.checked_add_signed(registers[reg.idx()]).unwrap();

                if let Some(instruction) = instructions.get_mut(toggled_index) {
                    *instruction = match *instruction {
                        Instruction::Dec(a) | Instruction::Tgl(a) => Instruction::Inc(a),

                        Instruction::Inc(a) => Instruction::Dec(a),

                        Instruction::Cpy(a, b) => Instruction::Jnz(a, b),

                        Instruction::Jnz(a, b) => Instruction::Cpy(a, b),
                    }
                }

                index += 1;
            }
        }
    }

    let a = registers[Register::A.idx()];

    dbg!(a);
}
