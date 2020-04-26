//! symbolを管理するモジュール。
//! symbolが被らないようにする。

pub struct SymbolManager {
    file_name: String,
    function_name: String,
    ifd_count: usize
}

impl SymbolManager {
    pub fn new() -> SymbolManager {
        SymbolManager {
            file_name: String::new(),
            function_name: String::new(),
            ifd_count: 0
        }
    }

    /// converterモジュールのifdマクロで使うsymbolを取得する
    pub fn get_ifd_symbol(&mut self) -> String {
        let s = format!("symbol-ifd-{}", self.ifd_count);
        self.ifd_count+=1;
        s
    }

    /// gotoのときに使うラベルを取得する
    pub fn get_goto_symbol(&self, label: &str) -> String {
        format!("symbol-goto-{}", label)
    }
}


#[cfg(test)]
mod test {
    use super::SymbolManager;

    #[test]
    fn test_symbol_manager() {
        SymbolManager::new();
    }

    #[test]
    fn test_symbol_manager_get_ifd_symbol() {
        let mut sm = SymbolManager::new();
        assert_eq!(&sm.get_ifd_symbol(), "symbol-ifd-0");
        assert_eq!(&sm.get_ifd_symbol(), "symbol-ifd-1");
    }
}