# planet6897 통합 검토 — 시스템 글꼴 공급

## Visual Sweep 메타파일 글꼴 공급

시스템 Fontconfig에서 지원 글꼴 2,069개를 자동 열거한 WASM 비교에서 실제 요청 문자에
필요하지 않은 메타파일 글꼴까지 등록해 64 MiB 상한을 초과했다. `sample16` HWPX와
기존 한컴 2024 Print의 전쪽 비교가 종료되지 않는 재현이었다.

메타파일과 그림자 텍스트의 글꼴 후보를 공통 glyph coverage로 선택한다. 같은 family의
분할 CFF 프로그램을 실제 공급 순서로 조사하고, CFF가 없을 때는 공급되지 않는 두 번째
TTF를 후보로 인정하지 않는다. cmap에는 있지만 표시 윤곽이 비어 있는 기호도 공급
완료로 취급하지 않는다. 시스템 글꼴 열거를 줄이거나 바이트 상한을 높이지 않는다.

Node 집중 검사 15건은 모두 통과했다. 같은 시스템 글꼴에서 HWPX 64쪽 WASM 비교도
종료 코드 0이며 #8397의 해당 문서 정밀 TSV보다 하락한 쪽은 없었다. 로그와 산출물은
`output/pr-review/planet6897-ordered-20261031/metafile-font-selection-final-tests.log`와
`output/pr-review/planet6897-ordered-20261031/check-sample16-metafile-candidate2-all/`에
있다. 글꼴 파일은 저장소에 추가하지 않는다.

전체 문서의 Native·WASM 정밀 대조와 선택된 실제 글꼴의 원인 분석은 별도로 계속한다.
위 64쪽 결과만으로 전체 통합 검토 완료나 다른 문서의 무하락을 주장하지 않는다.
