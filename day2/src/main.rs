use std::collections::HashMap;

const INPUT: &'static [&str] = &[
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
    let mut frequencies = HashMap::new();

    for (_, c) in word.chars().enumerate() {
        let frequency = frequencies.entry(c).or_insert(0);
        *frequency += 1;
    }

    frequencies
}

fn count_two_and_three_char_frequencies(words: &[&str]) -> (u32, u32) {
    let mut exactly_two = 0;
    let mut exactly_three = 0;

    for (_, item) in words.iter().enumerate() {
        let mut frequencies = character_frequencies(item);

        if frequencies.values().any(|&x| x == 2) {
            exactly_two += 1;
        }

        if frequencies.values().any(|&x| x == 3) {
            exactly_three += 1;
        }
    }

    (exactly_two, exactly_three)
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

}

fn main() {
    let (exactly_two, exactly_three) = count_two_and_three_char_frequencies(INPUT);

    println!(
        "exactly_two={}, exactly_three={}",
        exactly_two, exactly_three,
    );
}
