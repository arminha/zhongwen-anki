use once_cell::sync::Lazy;
use unicode_segmentation::UnicodeSegmentation;

use std::borrow::Cow;
use std::collections::HashSet;

fn split(text: &str) -> Vec<&str> {
    text.split_word_bounds().flat_map(split_word).collect()
}

fn split_word(word: &str) -> Vec<&str> {
    // TODO: there are some edge cases not handled by this but it should be safe for our use
    let mut result = Vec::with_capacity(2);
    let mut remaining = add_word(word, &mut result);
    while let Some(r) = remaining {
        remaining = add_word(r, &mut result);
    }
    result
}

fn add_word<'a>(word: &'a str, result: &mut Vec<&'a str>) -> Option<&'a str> {
    if SILLABLE_SET.contains(&word.to_lowercase()) {
        result.push(word);
        return None;
    }
    let mut end = word.len() - 1;
    while end > 0 {
        if let Some(slice) = word.get(0..end) {
            if SILLABLE_SET.contains(&slice.to_lowercase()) {
                result.push(slice);
                return Some(&word[end..word.len()]);
            }
        }
        end -= 1;
    }
    result.push(word);
    None
}

static SILLABLE_SET: Lazy<HashSet<String>> = Lazy::new(generate_sillable_set);

// see https://en.wikipedia.org/wiki/Pinyin_table
const SILLABLE_TABLE: &[&str] = &[
        "zhi chi shi ri zi ci si",
        "a ba pa ma fa da ta na la ga ka ha zha cha sha za ca sa",
        "e me de te ne le ge ke he zhe che she re ze ce se",
        "ai bai pai mai dai tai nai lai gai kai hai zhai chai shai zai cai sai",
        "ei bei pei mei fei dei tei nei lei gei kei hei zhei shei zei sei",
        "ao bao pao mao dao tao nao lao gao kao hao zhao chao shao rao zao cao sao",
        "ou pou mou fou dou tou nou lou gou kou hou zhou chou shou rou zou cou sou",
        "an ban pan man fan dan tan nan lan gan kan han zhan chan shan ran zan can san",
        "en ben pen men fen den nen gen ken hen zhen chen shen ren zen cen sen",
        "ang bang pang mang fang dang tang nang lang gang kang hang zhang chang shang rang zang cang sang",
        "eng beng peng meng feng deng teng neng leng geng keng heng zheng cheng sheng reng zeng ceng seng",
        "er",
        "yi bi pi mi di ti ni li ji qi xi",
        "ya dia nia lia jia qia xia",
        "yo",
        "ye bie pie mie die tie nie lie jie qie xie",
        "yai",
        "yao biao piao miao fiao diao tiao niao liao jiao qiao xiao",
        "you miu diu niu liu jiu qiu xiu",
        "yan bian pian mian dian tian nian lian jian qian xian",
        "yin bin pin min nin lin jin qin xin",
        "yang biang diang niang liang jiang qiang xiang",
        "ying bing ping ming ding ting ning ling jing qing xing",
        "wu bu pu mu fu du tu nu lu gu ku hu zhu chu shu ru zu cu su",
        "wa gua kua hua zhua chua shua rua",
        "wo bo po mo fo duo tuo nuo luo guo kuo huo zhuo chuo shuo ruo zuo cuo suo",
        "wai guai kuai huai zhuai chuai shuai",
        "wei dui tui gui kui hui zhui chui shui rui zui cui sui ",
        "wan duan tuan nuan luan guan kuan huan zhuan chuan shuan ruan zuan cuan suan",
        "wen dun tun nun lun gun kun hun zhun chun shun run zun cun sun",
        "wang guang kuang huang zhuang chuang shuang",
        "weng dong tong nong long gong kong hong zhong chong shong rong zong cong song",
        "yu nü lü ju qu xu",
        "yue nüe lüe jue que xue",
        "yuan juan quan xuan",
        "yun lün jun qun xun",
        "yong jiong qiong xiong",
    ];

fn generate_sillable_set() -> HashSet<String> {
    let mut sillables = HashSet::new();
    for line in SILLABLE_TABLE {
        for sillable in line.split_whitespace() {
            sillables.insert(sillable.to_owned());
            for i in 0..6 {
                sillables.insert(format!("{sillable}{i}"));
            }
        }
    }
    sillables
}

pub fn numbers_to_marks(text: &str) -> String {
    let mut result = String::new();
    for sillable in split(text) {
        if SILLABLE_SET.contains(&sillable.to_lowercase()) {
            if let Some(tone) = tone_number(sillable) {
                let sillable = &sillable[0..sillable.len() - 1];
                let with_tone = add_tone(sillable, tone);
                result.push_str(&with_tone);
            } else {
                result.push_str(sillable);
            }
        } else {
            result.push_str(sillable);
        }
    }
    result
}

fn tone_number(sillable: &str) -> Option<u8> {
    match &sillable[sillable.len() - 1..sillable.len()] {
        "0" | "5" => Some(5),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        _ => None,
    }
}

const TONEMARKS_1: [&[char]; 5] = [
    &['a', 'e', 'A', 'E'],
    &['ā', 'ē', 'Ā', 'Ē'],
    &['á', 'é', 'Á', 'É'],
    &['ǎ', 'ě', 'Ǎ', 'Ě'],
    &['à', 'è', 'À', 'È'],
];

const TONEMARKS_2: [&[char]; 5] = [
    &['o', 'O'],
    &['ō', 'Ō'],
    &['ó', 'Ó'],
    &['ǒ', 'Ǒ'],
    &['ò', 'Ò'],
];

const TONEMARKS_3: [[char; 6]; 5] = [
    ['i', 'u', 'ü', 'I', 'U', 'Ü'],
    ['ī', 'ū', 'ǖ', 'Ī', 'Ū', 'Ǖ'],
    ['í', 'ú', 'ǘ', 'Í', 'Ú', 'Ǘ'],
    ['ǐ', 'ǔ', 'ǚ', 'Ǐ', 'Ǔ', 'Ǚ'],
    ['ì', 'ù', 'ǜ', 'Ì', 'Ù', 'Ǜ'],
];

fn add_tone(sillable: &str, tone: u8) -> Cow<str> {
    if tone == 5 {
        return sillable.into();
    }

    let mut chars: Vec<_> = sillable.chars().collect();
    if replace_first(&mut chars, &TONEMARKS_1, tone) {
        return chars.into_iter().collect();
    }
    if replace_first(&mut chars, &TONEMARKS_2, tone) {
        return chars.into_iter().collect();
    }
    // second vowel
    'outer: for c in chars.iter_mut().rev() {
        for (i, vowel) in TONEMARKS_3[0].iter().enumerate() {
            if vowel == c {
                *c = TONEMARKS_3[tone as usize][i];
                break 'outer;
            }
        }
    }
    chars.into_iter().collect()
}

fn replace_first(chars: &mut [char], marks: &[&[char]; 5], tone: u8) -> bool {
    for (i, vowel) in marks[0].iter().enumerate() {
        for c in chars.iter_mut() {
            if vowel == c {
                *c = marks[tone as usize][i];
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn split_keep_non_pinyin() {
        let parts: &[_] = &["Ni3", "hao3", " ", "ni3", " ", "hao3"];
        assert_eq!(split("Ni3hao3 ni3 hao3"), parts);
    }

    #[test]
    fn numbers_to_marks_simple() {
        assert_eq!(numbers_to_marks("ma"), "ma");
        assert_eq!(numbers_to_marks("ma0"), "ma");
        assert_eq!(numbers_to_marks("ma1"), "mā");
        assert_eq!(numbers_to_marks("ma2"), "má");
        assert_eq!(numbers_to_marks("ma3"), "mǎ");
        assert_eq!(numbers_to_marks("ma4"), "mà");
        assert_eq!(numbers_to_marks("ma5"), "ma");
        assert_eq!(numbers_to_marks("Wo3"), "Wǒ");
        assert_eq!(numbers_to_marks("Ni3hao3"), "Nǐhǎo");
    }
}
