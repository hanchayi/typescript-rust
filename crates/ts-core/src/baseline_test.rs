use std::fs;
use std::path::{Path, PathBuf};
use crate::{compile, CompileOptions};

#[derive(Debug)]
pub struct BaselineTestResult {
    pub test_name: String,
    pub passed: bool,
    pub differences: Vec<String>,
}

pub struct BaselineTestRunner {
    pub test_dir: PathBuf,
    pub baseline_dir: PathBuf,
}

impl BaselineTestRunner {
    pub fn new(test_dir: PathBuf, baseline_dir: PathBuf) -> Self {
        Self {
            test_dir,
            baseline_dir,
        }
    }

    pub async fn run_tests(&self, pattern: Option<&str>) -> Vec<BaselineTestResult> {
        let mut results = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.test_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "ts") {
                    let file_name = path.file_stem().unwrap().to_string_lossy();
                    
                    if let Some(pattern) = pattern {
                        if !file_name.contains(pattern) {
                            continue;
                        }
                    }
                    
                    let result = self.run_single_test(&path).await;
                    results.push(result);
                }
            }
        }
        
        results
    }

    async fn run_single_test(&self, test_file: &Path) -> BaselineTestResult {
        let test_name = test_file.file_stem().unwrap().to_string_lossy().to_string();
        
        // 编译测试文件
        let options = CompileOptions {
            target: "es5".to_string(),
            module: "commonjs".to_string(),
            source_map: false,
            file_name: test_file.to_string_lossy().to_string(),
        };
        
        match compile(test_file, &options) {
            Ok(result) => {
                // 比较输出与基线
                let baseline_file = self.baseline_dir.join(format!("{}.js", test_name));
                
                if baseline_file.exists() {
                    if let Ok(baseline_content) = fs::read_to_string(&baseline_file) {
                        if result.code == baseline_content {
                            BaselineTestResult {
                                test_name,
                                passed: true,
                                differences: Vec::new(),
                            }
                        } else {
                            BaselineTestResult {
                                test_name,
                                passed: false,
                                differences: vec![format!("JS output differs from baseline")],
                            }
                        }
                    } else {
                        BaselineTestResult {
                            test_name,
                            passed: false,
                            differences: vec![format!("Could not read baseline file")],
                        }
                    }
                } else {
                    // 创建新的基线文件
                    let _ = fs::write(&baseline_file, &result.code);
                    BaselineTestResult {
                        test_name,
                        passed: true,
                        differences: vec![format!("Created new baseline")],
                    }
                }
            }
            Err(_) => {
                BaselineTestResult {
                    test_name,
                    passed: false,
                    differences: vec![format!("Compilation failed")],
                }
            }
        }
    }
}