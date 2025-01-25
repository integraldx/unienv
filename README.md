# unienv

Unity3D를 cli 환경에서 실행하기 쉽게 만들기 위한 보조용 프로그램입니다.

- Unity 에디터 버전을 프로젝트로부터 자동으로 감지하고, 맞는 버전을 실행합니다.

## How to install

### Cargo

```shell
cargo install --git https://github.com/integraldx/unienv
```

현재 시점에서 crates.io를 통한 설치는 지원하지 않고 있습니다.

## How to use

### 유니티 에디터 실행

```shell
unienv --projectPath <Project Path> ...
```

### Config

Unity 에디터나, Unity Hub의 설치 경로가 각 운영체제 기본과 다를 경우, 설정을 통해 정상적으로 작동하도록 수정할 수 있습니다.
