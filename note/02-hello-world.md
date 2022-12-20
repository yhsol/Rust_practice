# 02-hello-world

---

```
> rustc main.rs
> .\main.exe
Hello, world!
```

이게 실행이 왜 안되지?

- => Windows 에서 만들었던 exe 파일이 있었던 것. mac 에서는 main 파일을 실행하면 됨.
  ```
  ./main
  ```

---

- main 함수는 특별합니다: 이것은 모든 실행가능한 러스트 프로그램 내에서 첫번째로 실행되는 코드입니다.
- println!은 러스트 매크로 (macro) 라고 불립니다. 만일 대신에 함수라고 불리려면, (! 없이) println으로 입력되었어야 할 것입니다.

---

- 컴파일과 실행은 개별적인 단계입니다

- 러스트 프로그램을 실행하기 전에, 여러분은 아래와 같이 rustc 커맨드를 입력하고 여기에 여러분의 소스코드를 넘기는 식으로 러스트 컴파일러를 사용하여 이를 컴파일해야 합니다:

  - `$ rustc main.rs`

- 컴파일을 성공적으로 한 뒤, 러스트는 실행가능한 바이너리를 출력합니다.
- Linux, macOS, 그리고 Windows의 파워쉘 상에서는 여러분의 쉘에 다음과 같이 ls 커맨드를 입력하여 이 실행 파일을 볼 수 있습니다:

  ```
  $ ls
  main  main.rs
  ```

  - => 기존에는 main.exe, main.pdb 가 생성 됐었는데 지금은 main 만 보이는 이유가 Linux, macOs, Windows 의 파워쉘 에서는 main 만 보이고 Windows 와 COM 에서는 main.exe, main.pdb 가 나오는구나.
