# PR #8346 리뷰 — fix(parser/hwp3): 찾아보기 표시를 공통 IR로 읽고, 암호 HWP3 문서의 쪽 배경·문단 간격·표 간격·하이퍼텍스트 상자를 한컴 변환본과 맞춘다 (#8304)

## 최종 판정

**머지 보류 — 재체리픽 후보의 검증 진행 중.** 최신 base의 기능 보존, 관련 검증과 기존 정밀 TSV 하락 해소를 완료한 뒤 다시 판정한다.

## 접수와 적용

- 원 PR: [#8346](https://github.com/edwardkim/rhwp/pull/8346), 작성자 `planet6897`, reviewer `jangster77`.
- 원 PR head 조회값: `923ae70036a6b6e5ce94bf9c45cd2f8326cf8551`. 현재 원 PR head는 기여자 마지막 커밋 뒤의 devel merge이며 merge commit은 제외했다.
- 마지막 적용 원본: `75f167cee39b69f10d1eff25e5f9f55b18e64783`. 검증 시작 당시 코드 후보: `abea5b028f8f135bdf000ca700e1993ebf0b1973`, 기준 `upstream/devel`: `bc9dfdd057258de59afebeb5121c4bee7398e07f`.
- 관련 이슈: [#8304](https://github.com/edwardkim/rhwp/issues/8304).
- 선행 PR: 별도 누적 선행 PR이 명시되지 않았다.

후속 `e1f03ffbb3`은 Python 테스트 준비, `af175315e7`과 `e920cba6b0`은 회귀 검사 주석, `a63bb719de`는 검증 경과를 기록했다. `5d1294811b`은 전체 그림의 잘림 오판을 보정했다. `e1b8ab0f54`, `d6fe30c3ad`, `0897582f6c`, `8c4eca265e`에 Native 진단, #8410 집중 6건 통과와 글꼴 대조를 기록했으며 새 코드의 전쪽 출력 검증이 필요하다.

## 변경 주장과 검토 관점

HWP3 찾아보기 표시를 읽고 암호 문서의 배경·간격·표·하이퍼텍스트 처리를 보정한다.
사용자가 검토 범위에서 제외한 sample16 및 편람을 복원하지 않고 적용 대상만 확인한다.

## 체리픽과 보정

- 원본 `e33936a264f866cd8e7e43890962eb10eb67c5a6` → 통합 `e2222e48890db0fd4dd3b4957aca3be507996752`.
- 원본 `42b992cd1288db2c3fa3bc8dbf007a1524805cdc` → 통합 `39656f5b783a297ffe83d02d4af43f15f02bd1ae`.
- 원본 `75f167cee39b69f10d1eff25e5f9f55b18e64783` → 통합 `b5aa0c7e9489190a7cea82783e45f895f9628bca`.

`src/parser/hwp3/mod.rs` 충돌은 이전 보정 당시와 양쪽 Git blob이 동일해 기존 해결본을 재사용했다.

## 검증과 증거

이전 통합 후보의 최소 글꼴 전체 회귀 로그에서 이 PR 관련 다음 모듈의 실행을 확인했다. 새 base 후보의 재실행 결과는 이 값으로 대신하지 않는다.
- [`issue_8304_hwp3_index_mark.rs`](../../../tests/cases/issue_8304_hwp3_index_mark.rs): 이전 실행 7건 통과, 0건 실패, 0건 건너뜀.

검토 후보에 포함된 입력 바이트를 Git blob과 대조했다. 아래 자료는 원 PR의 제출 증거이며 새 후보의 실행 결과를 뜻하지 않는다.
- [`HWP3-password-123456.hwp`](../../../samples/HWP3-password-123456.hwp) — SHA-256 `db743d084efc9e08e839a5b4d978b16b8676434011776e090e4cda43e57304be`.
- [`HWP5-nopassword-123456.hwp`](../../../samples/HWP5-nopassword-123456.hwp) — SHA-256 `a34ecb8cde85b6db49c64a954cb7fa5d23b5f49367bc4753c90bfe89a075b50d`.
- [`HWP3-password-123456.pdf`](../../../pdf/HWP3-password-123456.pdf) — SHA-256 `3ced5ad95ad30331e2756b5b34509c1ac91dfe3c72013c8e14f2556ca6bd5776`.

기여자가 제출한 기준 자료: [`h3_hwp5_base_silhouette.tsv`](../../../mydocs/pr/assets/issue_8304/native/h3_hwp5_base_silhouette.tsv). 해당 자료만으로 새 통합 후보의 시각 검증을 통과 처리하지 않는다.

기존 생성 목록으로 최소 글꼴 회귀 12,320건 통과·45건 건너뜀을 확인했다. 생성 목록에서 누락된 #8410 두 모듈의 6건을 목록 재작성 후 같은 환경에서 별도로 실행해 모두 통과했다. 두 실행을 구분하며 한 번의 전체 실행으로 기록하지 않는다.

2026-11-01 고정 후보의 #8389·#8397 전쪽 정밀 대조에서 2,153행 중 Native 12행, WASM 8행이 공개 기준보다 낮았다. 이 수치는 이후 GIF·Canvas 보정 전 결과이며 최종 후보의 재검증 결과가 아니다. #7905 3쪽은 GIF 오른쪽 경계 누락, sample16 35쪽 WASM은 메타파일 내부의 미대응 3픽셀이 확인됐다. #8040 두 원본은 Native·WASM 전쪽이 90% 이상이나 Native 정밀 기준 하락이 남는다. 진행과 근거는 [작업 기록](../../working/task_m100_planet6897_system_fonts_20261031.md)을 따른다. 하락 보정과 전쪽 대조, 각 PR의 독립 Print·Native/fresh WASM 및 직접 판독을 완료해야 한다. #8410·#8412의 새 base 기능도 별도로 보존 검증한다.

원 PR의 최신 head·required check와 통합 후보의 새 검증 결과를 확인하기 전에는 승인이나 merge를 진행하지 않는다.
