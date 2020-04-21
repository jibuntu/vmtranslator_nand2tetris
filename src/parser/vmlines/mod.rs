use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;

/// 不要な行やコメントを削除したデータを提供する
pub struct Vmlines<R> {
    vm: BufReader<R>
}

impl<R: Read> Vmlines<R> {
    pub fn new(stream: R) -> Vmlines<R> {
        Vmlines {
            vm: BufReader::new(stream)
        }
    }

    /// 不要な行やコメントを除外して上で次の行を返す
    pub fn next(&mut self) -> Option<String> {
        let mut vmline = String::new();

        // 不要な行や空白を除外する
        loop {
            vmline.clear();
            if self.vm.read_line(&mut vmline).unwrap() == 0 {
                return None;
            }

            let mut line = vmline.as_str();
            let comment: Vec<_> = line.match_indices("//").collect();
            if comment.len() != 0 {
              line = &line[..comment[0].0];
            }

            // 両端の空白や改行を削除
            line = line.trim_matches(' ').trim_matches('\n');

            if line.len() == 0 {
                continue;
            }

            return Some(line.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vmlines;

    #[test]
    fn test_vmlines_next() {
        let mut lines = Vmlines::new("".as_bytes());
        assert_eq!(lines.next(), None);
    
        let mut lines = Vmlines::new(r#"
        // A
        VM
        "#.as_bytes());
        assert_eq!(lines.next(), Some("VM".to_string()));
        assert_eq!(lines.next(), None);
    
        let mut lines = Vmlines::new(r#"
        VM
        // A
        VM2 // VM
        "#.as_bytes());
        assert_eq!(lines.next(), Some("VM".to_string()));
        assert_eq!(lines.next(), Some("VM2".to_string()));
        assert_eq!(lines.next(), None);
    }
}
