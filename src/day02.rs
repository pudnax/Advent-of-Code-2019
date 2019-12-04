use crate::bail;
use crate::error::Error;
use std::io::BufRead;

pub fn run<R>(mut input: R) -> Result<(), Error>
where
    R: BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    let mut reader = std::io::BufReader::new(&content[..]);
    run_part(&mut reader, part_one)?;

    let mut reader = std::io::BufReader::new(&content[..]);
    run_part(&mut reader, part_two)?;
    Ok(())
}

pub fn run_part<F, R>(input: &mut R, func: F) -> Result<(), Error>
where
    R: BufRead,
    F: Fn(Vec<usize>) -> Result<(), Error>,
{
    let mut line = String::new();

    while input.read_line(&mut line)? > 0 {
        let prog = line
            .split(",")
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        func(prog)?;
        line.clear();
    }

    Ok(())
}

fn part_one(mut prog: Vec<usize>) -> Result<(), Error> {
    for i in (0..).step_by(4) {
        let o = prog[i + 3];

        match prog[i] {
            1 => prog[o] = prog[prog[i + 1]] + prog[prog[i + 2]],
            2 => prog[o] = prog[prog[i + 1]] * prog[prog[i + 2]],
            99 => break,
            _ => bail!("invalid op code"),
        }
    }

    println!("{:?}", prog);
    Ok(())
}

fn part_two(prog: Vec<usize>) -> Result<(), Error> {
    let run = |ops: &mut [usize], a: usize, b: usize| {
        ops[1..=2].copy_from_slice(&[a, b]);

        for i in (0..).step_by(4) {
            match ops[i] {
                1 => ops[ops[i + 3]] = ops[ops[i + 1]] + ops[ops[i + 2]],
                2 => ops[ops[i + 3]] = ops[ops[i + 1]] * ops[ops[i + 2]],
                99 => return Ok(ops[0]),
                _ => bail!("invalid op code"),
            }
        }

        bail!("end of ops")
    };

    let (n, v) = (0..100)
        .flat_map(move |n| (0..100).map(move |v| (n, v)))
        .find(|&(n, v)| run(&mut prog.clone(), n, v).ok() == Some(19690720))
        .unwrap();
    println!("{}", 100 * n + v);

    Ok(())
}
