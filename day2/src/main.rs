use std::collections::HashMap;

const PART_1_INPUT: &'static [&str] = &[
    "rvefnvyxzbodgpnpkumawhijsc",
    "rvefqtyxzsddglnppumawhijsc",
    "rvefqtywzbodglnkkubawhijsc",
    "rvefqpyxzbozglnpkumawhiqsc",
    "rvefqtyxzbotgenpkuyawhijsc",
    "rvefqtyxzbodglnlkumtphijsc",
    "rwefqtykzbodglnpkumawhijss",
    "rvynqtyxzbodglnpkumawrijsc",
    "rvefqtyxlbodgcnpkumawhijec",
    "rvefqtyxzbodmlnpnumawhijsx",
    "rvefqtyxzbqdbdnpkumawhijsc",
    "rvefqtyxzlodblnpkuiawhijsc",
    "rvefqtyizrodelnpkumawhijsc",
    "rveffjyxzgodglnpkumawhijsc",
    "rvefqjyxzbodalnpkumadhijsc",
    "rvefqtidzbodglnpkumawhdjsc",
    "hvefqtygzbodglnpkumawhijfc",
    "rzefqtyxzbodglfhkumawhijsc",
    "rmefqtyxzbolglnpkumaehijsc",
    "rnefqqyxzbodglnhkumawhijsc",
    "rvwfqvyxzbodglnpcumawhijsc",
    "rvefqtyxzbokgltpkumavhijsc",
    "rvefciyxzbodglnmkumawhijsc",
    "rvefptyxzbodglnpkuhashijsc",
    "rvefqtyxzrodglnpkxmawhiqsc",
    "rvefqtyxzbotglnpkumawriwsc",
    "rvufqtyxzbodglnplumawhijvc",
    "rvefutykzbodglnpkumaahijsc",
    "rvefqtyxqbodgllprumawhijsc",
    "rvegqttxzbodgllpkumawhijsc",
    "dvefqtyxzsodglnpkumawdijsc",
    "rvefqtyxkbodglnfkumawhijsj",
    "rvefqtyxzbodnlnpcumawhijnc",
    "rvefqtyxzbodglfpkuocwhijsc",
    "rvecqtyxzbbdganpkumawhijsc",
    "rvefytyxzbodglnpkubgwhijsc",
    "rvefxtyazbomglnpkumawhijsc",
    "rvefqgyxzbodglnpkumawyiksc",
    "avefqtyxzbodglnfkummwhijsc",
    "fvefqtyxzbbdglnpkumswhijsc",
    "rvefqtyxzxodglnpkumuuhijsc",
    "rvezqtyxzbydclnpkumawhijsc",
    "rvefqtyxzbohglnpkumawdijjc",
    "rvejqtyxzbodrlnpkumawhijsd",
    "rvefitzxzbxdglnpkumawhijsc",
    "rvefutyxzbvdglnikumawhijsc",
    "rvefqtyazbodgqnbkumawhijsc",
    "rvefqtyxzbolglnpkwmajhijsc",
    "rvefqtyxzjodglnpgwmawhijsc",
    "rvefhtyxzbodglbpaumawhijsc",
    "mvexqtyxzbodglnpkumawrijsc",
    "rvefqtyxwbodglnpkumawhbxsc",
    "rvefqtyxzbodgsnpkudawsijsc",
    "rvwfqtyxzbonglnwkumawhijsc",
    "rvefqtyxzjodglnpkfmawhwjsc",
    "rvefqtyxzbodglntkumughijsc",
    "rvefctyxzbodglnpkumawhiwsx",
    "avefqtyvzbodglnpkumawhijsb",
    "rfefqtyxzlodglnphumawhijsc",
    "rvefqtyxzfowglnpkumaehijsc",
    "rvhfvtyxzbodgqnpkumawhijsc",
    "rfefqtyxzbodglapkumuwhijsc",
    "rvefqclxzbodglnzkumawhijsc",
    "qvefqtyxzbodglnckumcwhijsc",
    "rvefqtyxzkodglnpkymawgijsc",
    "rvefqtyxzbodgfnpkumafhizsc",
    "rvefqtyxzbodglnxkumavhijsf",
    "rvevqtyxzbodgpnpkurawhijsc",
    "rvefqtyxziodglnpkubawhijss",
    "rrefqtpxzyodglnpkumawhijsc",
    "rvefqfyxzbodglcpkxmawhijsc",
    "rvefdtyxzbodglnpkumvwhijsn",
    "rverqtyxzbodglnpkwmawhijuc",
    "rvecjtyxzboxglnpkumawhijsc",
    "rvefqtyxzbodglnpkqmaxhifsc",
    "rtnfqtyxzbodglnpkumawhijmc",
    "lvefqtyxzbodelnpkumawhijsz",
    "dvefqtyxzbbdgvnpkumawhijsc",
    "rvefqlyhzbodglnpkumtwhijsc",
    "roefqtyxlbodglnpkumawhyjsc",
    "rvefqsydzjodglnpkumawhijsc",
    "rveybtyxzbodglnpkumawhijsn",
    "rvefqtyhzbodgvnpmumawhijsc",
    "rvefqxyazboddlnpkumawhijsc",
    "vvefqtyxzbohglqpkumawhijsc",
    "reefhtyxzbodglnpkkmawhijsc",
    "rvefqtyxzbodglnpkulowhijrc",
    "rveqqtyxzbodgknpkumawhijsk",
    "jvefqtqxzbodglnpkumawiijsc",
    "rvefqtyxzboxglnpvuqawhijsc",
    "rvefquyxzbodglwwkumawhijsc",
    "rvefqtyxzbodnlnpkumawhgjbc",
    "rvdfqthxdbodglnpkumawhijsc",
    "rvefqtyxzbodllnpkumawhujsb",
    "evefqtyxzboyglnpkumowhijsc",
    "rvefktyxzbomglnpzumawhijsc",
    "rvefqtyxzbodhlnnkrmawhijsc",
    "rvefqtyxrbodglnpkujaehijsc",
    "rvefqtyzzbodglnpkumrwhijsb",
    "evefqtyxzpodglfpkumawhijsc",
    "rvefqtyxibodglkpyumawhijsc",
    "rrefqtyxzbodglnpkudawhajsc",
    "rvifqtyxzbodglxpkumawhijlc",
    "rxefqtyxzbedglnpkumawhijsp",
    "rvnfqtyxzbopglnpkuqawhijsc",
    "rvefqtyxkbodglnpoumawoijsc",
    "dvefwtyxzbodglnpksmawhijsc",
    "rvkfqtyxzbodglnpkdmawhijsa",
    "rcefytyxzzodglnpkumawhijsc",
    "rvefqtkxzbodglnpktqawhijsc",
    "nvezqhyxzbodglnpkumawhijsc",
    "rrefqtyxzbodgunpkumpwhijsc",
    "rvefqtaxzbodgknpkumawhijic",
    "pvefqtyxzbodglnpkuxawsijsc",
    "rvefqtyxzbodglkpvumawhjjsc",
    "wvefqtyxzkodglnpkumawhhjsc",
    "rzefqtyxzbotglnpkumawhxjsc",
    "rvefqtxpzbodglnpkumawzijsc",
    "bgefqtyxzbodglnpkrmawhijsc",
    "rvefqlyxzbodglnpkumilhijsc",
    "cbefqtyxzbodglnpkumawhiesc",
    "rvefqtyxzbydelnpkumahhijsc",
    "rvefntyxzbodglnpkumaehijsw",
    "rverqtyxztodglopkumawhijsc",
    "rvefqtyxzdodgwrpkumawhijsc",
    "rvefqtyxibodglnikumawhtjsc",
    "qvafqtyxzbodglnpkurawhijsc",
    "rvefqtyxwbodglnpaumawoijsc",
    "rvefqtyxzoodglndknmawhijsc",
    "rvdfqtlxzyodglnpkumawhijsc",
    "rvefqtyxzbodglngfumawhinsc",
    "rsefqtyxzbodglnpkumawhijek",
    "rvoestyxzbodglnpkumawhijsc",
    "svefqtyxzboaglnprumawhijsc",
    "rvefqtybzbodgwnpkumawwijsc",
    "rvefqtyxzdwdglnpkvmawhijsc",
    "rvlfqtyxzbodglnpkrmawhixsc",
    "rvefqtyxwbodglepkumawhijsd",
    "rvefqtbxzbodglnqkumawhijmc",
    "rvefqtzxzbodglnpkumuzhijsc",
    "rvefqtyxzbodglnpkumawzwnsc",
    "rvwfqtyxzboiglnpkumawhijsg",
    "rtehotyxzbodglnpkudawhijsc",
    "rvegqtyxzbodglnpyumawhijsl",
    "rvecqtyxzbsdglnpkumawhojsc",
    "rvefqtyxzbodmlnpkumaghijfc",
    "rvefqtyxzxodglnpkumanvijsc",
    "rvefqtyxzbodglnbiugawhijsc",
    "lvefqtlxzbodglnplumawhijsc",
    "rvefqtyxvbodglnpkumaldijsc",
    "rmefqtyxzbodgvnpkuuawhijsc",
    "rvefqtyxzbodglnpkymeuhijsc",
    "rvefqtyxzuodganpsumawhijsc",
    "rxefqtyxzbodglnpkumgwhijwc",
    "rvefgtyxzbodglnpkudawxijsc",
    "ahefqtyxzbodglnpkumawhejsc",
    "rfefqtyxzbzdglnpkusawhijsc",
    "rvefqtyszqodgljpkumawhijsc",
    "rvefqtylzboiglnpkumrwhijsc",
    "rvefqtyxzltdglnpkumawhijsu",
    "rbefqtyxzbodglnpqumawhijsi",
    "rvefqtyozpodglnpkumawhijsa",
    "zvefqtyxzpopglnpkumawhijsc",
    "rvefqtyxzbodglnfkqmawhijsp",
    "rvefqtyxzbodgliakumawhijsf",
    "rvefqtymzrodgfnpkumawhijsc",
    "ivejqtyxzbodglnpkumawhijuc",
    "rvefqtyxzbodflnpkxwawhijsc",
    "dvrfqtyxzbodglnpkumashijsc",
    "rqefqtyxzbwdglnpkumawvijsc",
    "tvefqtkxzbodgltpkumawhijsc",
    "rvefdtyxzbodguxpkumawhijsc",
    "rveqqtyxvbodglnykumawhijsc",
    "rvefqtypzcovglnpkumawhijsc",
    "rvefqnyxzbosglnpkumdwhijsc",
    "rvefstjxzbodslnpkumawhijsc",
    "rvefqzyxzpodglnpkummwhijsc",
    "rvefqkyxzbodglnhgumawhijsc",
    "rvufqvyxzbodklnpkumawhijsc",
    "rvefotyxzhodglnpkumawhijsk",
    "rvefqtyxzbokglnpkumawvcjsc",
    "lvefqtyxzbolglnpkumawoijsc",
    "rvefqtywzoodglfpkumawhijsc",
    "rvehqtqxzbodglnpkumawhcjsc",
    "rqefqtyxzbodolnpkumjwhijsc",
    "rvefqtyxzbodglrpkunawgijsc",
    "rvefqtyxzbodglamkumawdijsc",
    "rvefvtyzzbodllnpkumawhijsc",
    "rvefqtyxzbldglnpfcmawhijsc",
    "rvefppyxzbodglnpkucawhijsc",
    "rvefquyuzbodglnpkumkwhijsc",
    "rvefqtyxzbodgqxpkumawhivsc",
    "rtefotyxzbodglnpkudawhijsc",
    "rvefqtyxzbodgbnmkuzawhijsc",
    "ivefqtyxzbodgsnpkumzwhijsc",
    "rvhfqtyxzbodolnpkumawhijsz",
    "rvefvtyxzbodwlnpkusawhijsc",
    "riemqtyxzbodglnpkumawhiasc",
    "rvtfqtyxzbqdglnpkumawuijsc",
    "raesqtyxzbodglnpkumawhijsj",
    "rvefqtyxzbodalmpkumawhihsc",
    "rvefqtlxzbodgznpkkmawhijsc",
    "rvefqbyxzbodglgpkubawhijsc",
    "rvefqtyxnbodgxnpkumswhijsc",
    "rvefqtyxzkodvlnukumawhijsc",
    "rvefqtyzzbocglnpkumafhijsc",
    "rvhfqtyxzbodglmpkumgwhijsc",
    "rvsfrtyxzbodnlnpkumawhijsc",
    "rvefqtyxzbxdglnpkujcwhijsc",
    "rvefqtyvzrodglnphumawhijsc",
    "reetatyxzbodglnpkumawhijsc",
    "rvefqtyxzbodglnpzumaoqijsc",
    "ovefqtyyzbodglnvkumawhijsc",
    "rvefqbyxzbodnlnpkumawhijsi",
    "xvefqtyxzbodgrnpkumawrijsc",
    "rvebqtyxzbodglnpkumazhiasc",
    "rqeretyxzbodglnpkumawhijsc",
    "rvefqtyxzyodglapkumvwhijsc",
    "rvesqxyxzbodglnpvumawhijsc",
    "rvefqtyxeborglnpkufawhijsc",
    "rvecqtyxzbodflnpkumawnijsc",
    "rvefdpyxtbodglnpkumawhijsc",
    "rvefqtyfzbodclnpkymawhijsc",
    "rvefqtywzbodglnpxumawhiusc",
    "rvefqtyxzbodglnpkumawzbjwc",
    "rvewqtyxdbodglnpxumawhijsc",
    "rvefqtyxzgocglnpkgmawhijsc",
    "rvufqtyxzbodggnpkuzawhijsc",
    "rvefqtynzlodgllpkumawhijsc",
    "rvedqtyxzbodghnpkumawhujsc",
    "rvefqtyxlbodgnnpkpmawhijsc",
    "rvefqtyxzboqglnpkzmawhijec",
    "rvefqtyxzbodglnpkfmwwyijsc",
    "rwefqtkxzbodzlnpkumawhijsc",
    "rvefqtyxvbodglnpkufawhyjsc",
    "rvefqtyxzbodgltpkumawhqmsc",
    "rvefctyxzbodglfpkumathijsc",
    "rvefqtyxzbodgfnpkuuawhijfc",
    "rvefqttxzbodglnpmumawhijwc",
    "rvefqtyxzbodglnpkqmawhihsj",
    "rvefqtyxzbsdglcnkumawhijsc",
    "rvbiqtyxzbodglnpkumawhijlc",
    "rnefqtylzvodglnpkumawhijsc",
    "mvefqtyxzbddglnpkumcwhijsc",
    "rvefwtyxzbodglnpkgmawhijxc",
    "rvefqtyxljodglnpkumxwhijsc",
    "rvefqtyxzbodglnpkuprwhijsd",
    "rcxfqtyxzbldglnpkumawhijsc",
    "rvetqtyxzbojglnpkumewhijsc",
    "rvxfqtyxzbtdglnpkbmawhijsc",
];

fn character_frequencies(word: &str) -> HashMap<char, u32> {
    word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_default() += 1;
        acc
    })
}

fn count_two_and_three_char_frequencies(words: &[&str]) -> (u32, u32) {
    let mut exactly_two = 0;
    let mut exactly_three = 0;

    words
        .iter()
        .map(|word| character_frequencies(word))
        .for_each(|frequencies| {
            if frequencies.values().any(|&x| x == 2) {
                exactly_two += 1;
            }

            if frequencies.values().any(|&x| x == 3) {
                exactly_three += 1;
            }
        });

    (exactly_two, exactly_three)
}

fn count_different_chars(a: &str, b: &str) -> u32 {
    let different = a.len() as u32;

    a.chars().zip(b.chars()).fold(
        different,
        |acc, (a_char, b_char)| {
            if a_char == b_char {
                acc - 1
            } else {
                acc
            }
        },
    )
}

fn differ_by<'a>(words: &[&'a str], num: u32) -> Option<(&'a str, &'a str)> {
    for a in words {
        for b in words {
            if count_different_chars(a, b) == num {
                return Some((a, b));
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_character_frequencies() {
        let input = "abbcccdddd";

        let mut expected = HashMap::new();
        expected.insert('a', 1);
        expected.insert('b', 2);
        expected.insert('c', 3);
        expected.insert('d', 4);

        let frequencies = character_frequencies(input);

        assert_eq!(frequencies, expected);
    }

    #[test]
    fn get_num_words_with_two_and_three_char_frequencies() {
        let input = &["aaXXXXXX", "bXXXXXXX", "cccXXXXX", "dddXXXXX"];

        let (two, three) = count_two_and_three_char_frequencies(input);
        assert_eq!(two, 1);
        assert_eq!(three, 2);
    }

    #[test]
    fn get_num_different_chars() {
        assert_eq!(count_different_chars("abcde", "axcye"), 2);
        assert_eq!(count_different_chars("fghij", "fguij"), 1);
    }

    #[test]
    fn find_words_with_one_different_char() {
        let input = &[
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];

        assert_eq!(differ_by(input, 1).unwrap(), ("fghij", "fguij"));
        assert_eq!(differ_by(input, 2).unwrap(), ("abcde", "axcye"));
    }
}

fn main() {
    let (exactly_two, exactly_three) = count_two_and_three_char_frequencies(PART_1_INPUT);

    // Part 1
    println!("Part 1 - result={}", exactly_two * exactly_three);

    // Part 2
    match differ_by(PART_1_INPUT, 1) {
        None => println!("nothing found!"),
        Some((word_a, word_b)) => println!("word_a={}, word_b={}", word_a, word_b),
    }
}
