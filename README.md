
### 1. 실행 방법

- env를 주입하기 위해 env_key_1=env_val_1 env_key_2=env_val_2 cargo run 명령어 사용

- ㅇ


### 2. TODO LIST

1. 각종 에러에 대한 핸들링
    1. aws env가 없는 경우 - panic 및 alert
    2. format이 잘못된 경우 - aws의 error message를 그대로 복사해서 사용하기
    3. error 시 반환값에 대한 핸들링
2. performance 테스트
    1. html template을 AWS 에 등록해서 사용하는 경우 / 별도로 사용하지 않는 경우
    2. send email을 단일로 여러개 보내는 경우 / 50개씩 보내는 경우
3. 반환타입에 대한 핸들링 (not sure)
    1. 전체 성공/실패 또는 N개는 성공하고 M개는 실패한 경우
    2. 참고: 어떤 이메일에 보내는게 성공하고, 어떤 이메일에 보낸게 실패했는지는 알기 어려움
4. 성공/실패에 대해 자세히 로깅하는 방법 및 비용 찾아내기
5. 성공/실패에 또는 에러에 대한 slack 메시지 연동 (not sure)
