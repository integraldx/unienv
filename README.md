# unienv

Unity3D를 cli 환경에서 실행하기 쉽게 만들기 위한 보조용 프로그램입니다.

- Unity 에디터 버전을 프로젝트로부터 자동으로 감지하고, 맞는 버전을 실행합니다.

## How to install

### Cargo

```shell
cargo install unienv
```

## How to use

### 유니티 에디터 실행

```shell
unienv editor -projectPath <Project Path> ...
```

### 유니티 허브 실행

```shell
unienv hub ...
```

### Config

Unity 에디터나, Unity Hub의 설치 경로가 각 운영체제 기본과 다를 경우, 설정을 통해 정상적으로 작동하도록 수정할 수 있습니다.

또한, Unity Accelerator 등 추가적으로 적용할 옵션이 있는 경우 커맨드에 추가할 수 있습니다.
