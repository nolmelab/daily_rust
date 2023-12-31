# 지난 7개월의 여정

`Rust in Action`의 한글 번역 책으로 3개월 정도 시스템 프로그래밍 기초를 다루면서 
러스트를 이해하는 공부를 진행했습니다. 재미있고 유익한 내용이 많았지만 돌아보면 
러스트를 처음 공부하기에는 좋은 접근은 아닌 듯 합니다. 

이후 Rust by example, 구글의 러스트 과정, 러스트 언어 책, 러스트노미콘, tokio 프로그래밍 
튜토리얼 등을 공부했습니다. 

## 공부하면서 어려웠던 부분들 

러스트의 학습 곡선은 가파르다고 다들 인정합니다. 보통 3~5개월 정도 시간이 걸리고 
주로 익숙하지 않은 새로운 접근 방법이 원인이긴 하지만 상당히 오래 걸리는 편입니다. 
꾸준히 해서 꼭 마스터 하겠다는 각오가 없으면 저처럼 혼자 공부하는 경우에는 더 어려울 듯 
합니다. 

### 소유, 소유권, 빌림, 빌림 검사, 수명

처음에 어렵다고 하는 부분입니다. 시작할 때 참조 자체는 C++의 레퍼런스와 const 레퍼런스랑 
비슷하다고 생각했습니다. 컴파일러가 수명을 확인하는 방법이 코드 실행 구조를 반영하기 때문에
범위와 다른 경우들이 있고 아직 완벽하기 않기 때문에 생기는 예외 부분들이 있어서 힘들었던 
것 같습니다. 

지금은 컴파일러가 잘 알아서 해주고, 이상하게 안 되는 부분이 있으면 오류 메세지를 보면서 
좀 더 간결한 구조로 바꾸면서 해결하면 된다고 생각하고 사용하고 있습니다. 

### trait, generic, OOP가 아님 

인터페이스에 해당하는 trait로 전체 구조를 만드는 강력한 힘을 갖고 있기 때문에 
trait, generic, trait bound, associative type (연관 타잎)을 사용하는 방법에 
익숙해져야 합니다. 

generic은 C#의 generic과 매우 비슷합니다. 단지, 수명도 generic의 파라미터 중 하나가 
되기 때문에 혼란스러울 떄가 있습니다. 수명을 명시해야 하는 경우를 컴파일러가 알려주기 
때문에 컴파일러에 기대어 구현을 하면서 이해를 높여 나가면 되는 듯 합니다. 

trait bound는 C#의 where 문이 하는 일을 훨씬 강력하고 상세하게 지정할 수 있는 기능입니다. 
심지어 트레이트가 다른 경우에 다른 동작을 하도록 구현할 수 있습니다. 이는 C++의 
템플릿 특수화와 가깝습니다. Bound를 하는 문법이 세 가지 정도 있기 때문에 처음에 
헷갈릴 수 있습니다. 

연관 타잎은 trait의 구현 내부에서 연결된 타잎을 구현하는 쪽에서 변경할 수 있도록 합니다. 
추가로 선택할 수 있는 타잎을 구현 쪽에 위임하기 때문에 유용한 경우가 많아집니다. 

러스트는 OOP 언어가 아닙니다. struct가 클래스가 아니기 때문에 그렇습니다. 포함하지 않고 
다른 struct의 변수들을 가져올 방법이 없습니다. 대신 trait의 확장 (상속과 같은)은 가능하고 
기본 구현도 가능하기 때문에 기능 자체에 대한 상속은 가능합니다. 

포함과 인터페이스 중심으로 최종적으로 동작하는 프로그램에서 필요한 trait를 찾아서 단위 
구조를 찾아 나가는 흐름으로 진행해야 하므로 처음에 생각하기 어렵고, 저도 아직 훈련 중에 
있습니다. 
### trait object 

trait object는 dyn 키워드로 지정되는 동적인 dispatching 기능입니다. C#이나 C++의 virtual 
함수를 지원하는 기능과 비슷하지만 trait에 대해서만 동작합니다. 

러스트 언어 책의 OOP 항목에 나와있는 내용 정도면 되고 생각보다 dyn 키워드는 큰 시스템에서도 
많이 찾아볼 수 없습니다. 주로 `Box<dyn Error>` 형태로 많이 보게 됩니다. 

### unsafe 러스트 

unsafe 쪽도 처음에 이해하기 어려운 부분이긴 합니다. 시간이 지나면서 포인터를 다루거나 
transmute와 같은 강력한 형 변환 등의 일반적인 구현 패턴들을 이해하면 크게 문제 되지 않는 
것으로 보입니다. 

unsafe는 프로그래머와 러스트 시스템 (컴파일러, 런타임) 간의 계약을 뜻 합니다. 프로그래머가 
러스트에 맞춰 안전하게 만들곘다는 약속입니다. 그리고, 이를 뒷받침하는 도구들이 있으므로 
필요한 경우 익혀 나가면 될 듯 합니다. 

## tokio에서 어려운 부분들 

tokio의 공식 튜토리얼 문서는 매우 잘 작성된 문서들입니다. 사실 거기에 있는 내용이 
필요한 전부라고 해도 과언이 아닙니다. 단지, 그렇기 때문에 깊이가 있고 처음에 이해했다고 
생각한 내용이 계속 다시 보게 되는 경우가 많습니다. 

tokio에서 가장 어려웠던 부분은 Pin 관련 내용이었습니다. 아직 익숙하게 사용할 정도로 
잘 이해했다고 보기 어려운 면이 있습니다. daily rust를 통해 정리할 계획입니다. 

단지, 자기 참조가 있는 struct의 경우 move 안정성을 보장할 수 없기 때문에 한번 Pin 된 경우 
Unpin이 아니라면 더 이상 옮기지 않는다는 걸 프로그래머가 보장하고, 컴파일러와 런타임에서 
이에 의지하여 도와줘서 안정성을 만든다는 정도로 이해했습니다. 

pin_project나 pin_project_lite에서 멤버 단위로 pin을 지정할 수 있게 한 부분이 어떻게 
동작하는지도 더 봐야 할 부분입니다. 

