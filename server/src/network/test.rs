use std::vec;

pub trait AutoGenFuzz<T, U>
where
    T: Clone,
{
    fn fuzz_a_packet(packet: T, skip_fuzzing: &Vec<U>) -> Vec<T>;
    fn fuzz_packets(packets: &Vec<T>, skip_fuzzing: &Vec<Vec<U>>) -> Vec<T> {
        let mut res = vec![];
        for (p, skip_fuzzing) in packets.iter().zip(skip_fuzzing) {
            res.append(&mut Self::fuzz_a_packet(p.clone(), skip_fuzzing));
        }
        res
    }

    fn generate_fuzzing(exec: &Vec<T>, skip_fuzzing: &Vec<Vec<U>>) -> Vec<Vec<T>> {
        let mut res = vec::Vec::new();

        for (i, e) in exec.iter().enumerate() {
            let prefix = &exec[..i].to_vec();
            let suffix = &exec[i + 1..].to_vec();
            let cell_to_fuzz = e;
            let fuzzed_cell = Self::fuzz_packets(&vec![cell_to_fuzz.clone()], skip_fuzzing);
            for c in fuzzed_cell {
                let mut output = vec![];
                output.append(&mut prefix.clone());
                output.push(c);
                output.append(&mut suffix.clone());
                res.push(output)
            }
            let mut output = vec![];
            output.append(&mut prefix.clone());
            output.append(&mut suffix.clone());
            res.push(output);
        }

        res
    }
}
