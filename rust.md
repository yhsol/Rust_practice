# rust

- Cargo

  - rust 의 빌드 시스템 및 패키지 매니저.
  - javasciprt 의 npm, yarn 비슷한 역할을 한다.
  - cargo run, cargo build 와 같은 방식으로 작동할 수 있다.

  - Cargo.toml
    - Cargo 의 환경설정 포맷.
    - package.json 과 비슷한 역할을 한다.

- Variables and Mutability
  - 기본 변수는 immutable.
  - let 역시 변경 불가.
  - mutable 을 원한다면 mut 키워드를 붙이면 된다.
    - let mut some_variables
- Differences Between Variables and Constants

  - 상수에 대해서는 mut 을 사용하는 것이 허용되지 않음.  
     즉, 상수는 기본 설정이 불변성인 것이 아니고 불변성 그 자체.
  - const 키워드 사용하며, 값의 유형을 선언해야 한다.
  - 상수는 전체 영역을 포함하여 어떤 영역에서도 선언될 수 있음.
  - 상수는 오직 상수 표현식만 설정될 수 있지, 함수 호출의 결과값이나 그 외에 실행 시간에 결정되는 값이 설정될 수는 없다
  - rust 에서는 상수 명명 규칙에 따라 모든 단어를 대문자로 사용한다.

- Shadowing
  - 변수를 shadows 할 수 있다.

```
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

- mut 과 몇가지 차이점이 있다.

  - shadows 는 변수의 타입 역시 변경 가능하다.
  - 즉, 새로운 변수가 되는 것.
  - 그러면서도 이름을 그대로 쓸 수 있다는 장점이 있다.

- Data Types

  - rust 에서 사용되는 모든 값(value)들은 어떤 타입을 갖는다.
  - 그렇기 때문에 어떤 형태의 데이터인지 명시하여, rust 에게 알려줘서 이를 통해 데이터를 어떻게 다룰지 알 수 있도록 해야 한다.

  - rust 는 타입이 고정된 언어이다.(Rust is a statically typed language)
  - 이게 의미하는 바는 모든 변수의 타입이 컴파일 시에 반드시 정해져 있어야 한다는 것.

- Scalar Types

  - integers, floating-point numbers, Booleans, and characters.

  - Integer Types

    - 소수점이 없는 숫자.
    - 부호, 미부호 (e.g. i8, u8)
    - i8, u8 은 8-bit 의 크기를 가지며,  
       i32, u32 는 32-bit 의 크기를 갖는다.
    - 부호 혹은 미부호의 의미는, 숫자가 양수 혹은 음수를 다룰 수 있는지 혹은 없는지를 나타냅니다.
    - isize와 usize타입은 당신의 프로그램이 동작하는 컴퓨터 환경이 64-bits인지 아닌지에 따라 결정됩니다. 64-bit 아키텍처이면 64bit를, 32-bit 아키텍처이면 32bit를 갖게 됩니다.
    - isize나 usize는 주로 콜렉션의 정렬을 색인할 때 사용됩니다.
    - 확실하게 정해진 경우가 아니면 Rust의 기본 값인 i32가 일반적으로는 좋은 선택입니다.

  - Floating-Point Types

    - f32, f64 가 있으며 각기 32bit 와 64bit 의 크기를 갖는다.
    - 기본 타입은 f64.

  - Numeric Operations

    - 수학적 연산은 모든 숫자 타입에 적용됨.
    - +, -, \*, /, %

  - The Boolean Type

    - true 와 false 둘 중 하나의 값만 가질 수 있다.
    - boolean 타입은 bool 로 명시된다.

  - The Character Type
    - Rust의 char는 이 언어의 가장 근본적인 알파멧 타입.
    - 스트링은 큰따옴표,  
      char 타입은 작은따옴표를 쓴다.

- Compound Types

  - 복합 타입들은 다른 타입의 다양한 값들을 하나의 타입으로 묶을 수 있습니다. Rust는 두 개의 기본 타입들을 갖고 있습니다: 튜플과 배열.

  - The Tuple Type
    - 튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법.
    - 괄호 안에 콤마로 구분되는 값들의 목록을 작성하여 튜플을 만든다.
    - 튜플에 포함되는 각 값의 타입이 동일할 필요없이 서로 달라도 된다.

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

- 튜플은 단일 요소를 위한 복합계로 고려되었기에 변수 tup에는 튜플 전체가 bind 됩니다. 개별 값을 튜플의 밖으로 빼내오기 위해서는, 패턴 매칭을 사용하여 튜플의 값을 구조해체 시키면 됩니다. 다음을 봅시다:
- 패턴 매칭은 javascript 의 비구조화 할당과 비슷한 역할을 하는 듯 하다.

```
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

- 패턴 매칭을 통한 구조해체에 추가로, 우리는 마침표(.) 뒤에 우리가 접근하길 원하는 값의 색인(index)을 넣는 것을 통해 튜플의 요소에 직접적으로 접근할 수 있습니다. 예제를 봅시다:

```
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

- index 는 0 부터 시작.

- The Array Type

  - 여러 값들의 집합체를 만드는 다른 방법은 배열이다.
  - 튜플과는 다르게, 배열의 모든 요소는 모두 같은 타입이여야 한다.
  - rust 의 배열이 몇 다른 언어들의 배열과 다른 점은 rust 에서는 배열은 고정된 길이를 갖는다는 점이다.
  - 한번 선언되면, 이들의 크기는 커지거나 작아지지 않는다.
  - 대괄호([]) 안에 값들을 콤마로 구분하여 나열해서 배열을 만든다.
  - 배열이 유용할 때는 데이터를 heap 보다 stack 에 할당하는 것을 원하거나,  
    항상 고정된 숫자의 요소를 갖는다고 확신하고 싶을때이다.
  - 벡터 타입과는 다르게 가변적이지 않다.
  - 벡터 타입은 유사 집합체로 표준 라이브러리에서 제공되며 확장 혹은 축소가 가능하다.
  - 배열이나 벡터 중에 뭘 선택해야 할지 확실하지 않은 상황이라면 벡터를 사용하는 것이 좋다.

  - Accessing Array Elements
    - index 를 통해서 배열의 요소에 접근할 수 있다.

```
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

- Invalid Array Element Access

  - 유효하지 않은 배열 요소에 접근하려 한다면 rust 는 panic 한다.
  - rust 는 잘못된 index 를 제공할 경우 메모리 접근을 허용하고 계속 진행하는 대신 즉시 종료하여 이러한 종류의 오류로부터 사용자를 보호한다.

- Functions
  - main 함수
    - 다수의 프로그램에서 실행 지점이다.
  - fn 키워드를 통해 함수의 선언을 한다.

```
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

- 위의 예제에서 보듯이 함수가 프로그램 내에 정의되어 있다면 함수의 정의 위치는 상관 없다.

- Function Parameters

```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

- 함수의 선언부에서, 여러분은 반드시 각 매개변수의 타입을 정의해야 합니다. 이 사항은 Rust를 설계하며 내린 신중한 결정사항입니다: 함수의 정의에 타입을 명시하여 코드 내 다른 부분에서 이들을 사용하는 것을 통해 당신의 의도를 추측하지 않아도 되게 됩니다.

여러분의 함수에 여러 개의 매개변수를 사용하고 싶으면, 매개변수들을 다음처럼 쉼표와 함께 구분해서 사용할 수 있습니다:

- 매개변수가 다 같은 타입은 아니여도 된다.

```
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```
