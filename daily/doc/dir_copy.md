# 폴더 순회와 파일 복사 

오늘은 러스트 공부를 하면서 개인적으로 역사적인 날입니다. 러스트 CLI 툴을 만들어
업무에 사용한 첫 날입니다. 

특정 폴더에서 하위 폴더를 포함하여 변경된 파일들을 뽑아서 별도의 폴더로 복사하는 간단한
툴입니다. 간단하지만 그리 쉽지만은 않은 기능이기도 합니다. 왜냐하면 플래폼 호환이 되면서 
안정적으로 파일 시스템에 대한 조회와 처리를 할 수 있는 API가 잘 제공되어야 하기 때문입니다. 

러스트는 여러 플래폼에서 동작하도록 컴파일러와 런타임에서 미리 잘 준비하여 기능을 제공하고 
있습니다. 파이썬을 공부할 때도 비슷한 툴을 만든 적이 있는데 생각보다 안정적으로 동작하지는 
않았던 것으로 기억합니다 (잘 몰라서 그랬을 수도 있습니다). 

러스트로 구현할 때 마치 파이썬으로 구현하는 것처럼 수월하게 진행되었고, 이 과정을 
별도의 검색 없이 std::fs와 std::path 등의 라이브러리 소스 파일의 문서 내용을 읽고 
몇 가지 연습을 한 후에 바로 구현할 수 있었습니다. 타잎 체킹이나 소유권 관련 컴파일러 
오류를 보면서 맞춰 주고 나니 잘 동작했습니다. 

```rust
//! 분 단위로 이전 시간을 지정하여 그 이후에 변경된 파일들을 특정 폴더로 복사한다.
//! 이 기능이 있으면 vcpkg 설치 후 새로 추가된 파일들만 복사 가능하다.
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// src 폴더를 하위폴더까지 뒤져서 dst의 폴더의 하위 폴더에 복사한다. 
/// mb는 minutes before로 이 시간 이후에 변경된 파일만 복사한다.
pub fn copy<P: AsRef<Path>>(src: P, dst: P, mb: u32) {

    // dst와 path를 같은 바운드를 할 경우 재귀 호출에서 오류가 나와서 맞춤
    // 특정 디렉토리의 하위 디렉토리를 순회하면서 파일에 대해 작업
    fn for_each_dir<P: AsRef<Path>, Q: AsRef<Path>>(dst: &P, path: Q, after: &SystemTime, dir_modified: bool) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let meta = entry.metadata().unwrap();
                    if meta.is_dir() {
                        let modified_time = entry.metadata().unwrap().modified().unwrap();
                        let modified = modified_time >= *after;
                        for_each_dir(dst, entry.path(), after, modified);
                    } else {
                        let modified_time = entry.metadata().unwrap().modified().unwrap();
                        if dir_modified || modified_time >= *after {
                            copy_file(dst, &entry);
                        }
                    }
                }
            }
        }
    }

    // create_dir_all() 함수는 생각보다 참 편리한 함수. 
    fn copy_file<P: AsRef<Path>>(dst: &P, entry: &fs::DirEntry) {
        let mut to = PathBuf::new();
        to.push(dst.as_ref());
        to.push(entry.path().parent().unwrap());
        let _result = fs::create_dir_all(to.as_path());

        to.push(entry.path().file_name().unwrap());
        let result = fs::copy(entry.path(), &to);

        // TODO: canonicalize보다 익숙한 형식의 변환이 필요하다.
        // canonicalize()는 윈도우의 경우 host 항목을 포함한다.
        // let to = to.canonicalize().unwrap();

        match result {
            Err(e) => {
                println!(
                    "Error: {:?}. Failed to copy {:?} to {:?}",
                    e,
                    entry.path(),
                    to.as_path()
                )
            }
            _ => { println!("Copy to {:?}", &to); }
        }
    }

    let now = SystemTime::now();
    let duration = Duration::from_secs((mb * 60) as u64);
    // SytemTime과 Duration 간 연산이 정의되어 있다. 세심한 구현이다. 커뮤니티의 힘이다.  
    let before = now - duration;

    for_each_dir(&dst, src, &before, false);
}
```