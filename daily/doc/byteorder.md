# byteorder 

엔디안과 바이트 순서가 모호할 때가 많았다. 러스트의 byteorder 크레이트로 
간단한 예제를 실행해 보고 명확해졌다. 

```rust
//! byteorder에 대해 살펴보면서 추가로 이해를 더한다.
//!
//! 러스트는 byteorder와 같이 간결하면서 명확한 코드를 갖고 있다. 
//! C++에서 이런 수준으로 간결하면서 명확하게 정리한 라이브러리를 만나기 어려웠다. 
//! 커뮤니티 전체가 하나로 빠르게 함꼐 성장하는 생태계라 그런 것 같다. 
//! 이를 언어가 소스 공유를 필요로 하기 때문에 강제하고, 컴파일러가 단일하며 
//! MIT와 아파치 라이센스라 가능하게 되었다. 
#[cfg(test)]
mod tests {

    use byteorder::{BigEndian, ReadBytesExt};
    use byteorder::{LittleEndian, WriteBytesExt};
    use std::io::Cursor;

    #[test]
    fn tr_read() {
        // vec!의 순서에서 앞 쪽이 낮은 주소이고 뒤 쪽이 높은 주소이다.
        let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
        // Note that we use type parameters to indicate which kind of byte order we want!
        assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
        assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
        println!("{:b}", 2);
        println!("{:b}", 5);
        println!("{:b}", 3);
        println!("{:b}", 0);
        println!("{}", 0x0205); //
        println!("{}", 0b0000001000000101); //
        println!("{}", 0x0300); //
        println!("{}", 0b000001100000000); //
    }

    #[test]
    fn tr_write() {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(517).unwrap();
        wtr.write_u16::<LittleEndian>(768).unwrap();
        assert_eq!(wtr, vec![5, 2, 0, 3]);

        let mut rdr = Cursor::new(wtr);
        assert_eq!(517, rdr.read_u16::<LittleEndian>().unwrap());
    }
}
```

이 보다 더 명확할 수 없다.

C++에서는 align이 있으므로 serialize / deserialize할 때 주의해야 한다.

그 외에 엔디안은 명확하다. 

