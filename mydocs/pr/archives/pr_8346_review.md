# PR #8346 리뷰 — fix(parser/hwp3): 찾아보기 표시를 공통 IR로 읽고, 암호 HWP3 문서의 쪽 배경·문단 간격·표 간격·하이퍼텍스트 상자를 한컴 변환본과 맞춘다 (#8304)

## 최종 판정

**머지 보류 — 재체리픽 후보의 검증 진행 중.** 최신 base의 기능 보존, 관련 검증과 기존 정밀 TSV 하락 해소를 완료한 뒤 다시 판정한다.

## 접수와 적용

- 리베이스 후 코드 후보: `1f8e948ab443c6f963413e45eb330aab1294ca9f`, 기준 `upstream/devel`: `da39c536613a4bd543dc72b63a556183efc0383f`. 체리픽 대응표에는 새 SHA와 리베이스 전 SHA를 함께 기록했다.
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

- 원본 `e33936a264f866cd8e7e43890962eb10eb67c5a6` → 통합 `b9affc985f058cb677b0be6d0899df531f1e633c` (리베이스 전 `e2222e48890db0fd4dd3b4957aca3be507996752`).
- 원본 `42b992cd1288db2c3fa3bc8dbf007a1524805cdc` → 통합 `d4807c6a9a801ffeb78a3b1a7ff0fba6b5c8e5ee` (리베이스 전 `39656f5b783a297ffe83d02d4af43f15f02bd1ae`).
- 원본 `75f167cee39b69f10d1eff25e5f9f55b18e64783` → 통합 `dc59b5e5bc40bbc3948d3e0c96c54afe0e4ac58f` (리베이스 전 `b5aa0c7e9489190a7cea82783e45f895f9628bca`).

`src/parser/hwp3/mod.rs` 충돌은 이전 보정 당시와 양쪽 Git blob이 동일해 기존 해결본을 재사용했다.

이번 검토 범위에서 제외한 sample16을 직접 열던 빈 하이퍼텍스트 대조 함수는 제거했다. 암호 HWP3 원본을 사용하는 찾아보기·쪽 배경·본문 검사들은 유지한다. 제거한 함수가 보호하던 빈 상자 반례는 이 PR의 독립 회귀 근거로 주장하지 않는다.

## 검증과 증거

이전 통합 후보의 최소 글꼴 전체 회귀 로그에서 이 PR 관련 다음 모듈의 실행을 확인했다. 새 base 후보의 재실행 결과는 이 값으로 대신하지 않는다.
- [`issue_8304_hwp3_index_mark.rs`](../../../tests/cases/issue_8304_hwp3_index_mark.rs): 이전 실행 7건 통과, 0건 실패, 0건 건너뜀.

검토 후보에 포함된 입력 바이트를 Git blob과 대조했다. 아래 자료는 원 PR의 제출 증거이며 새 후보의 실행 결과를 뜻하지 않는다.
- [`HWP3-password-123456.hwp`](../../../samples/HWP3-password-123456.hwp) — SHA-256 `db743d084efc9e08e839a5b4d978b16b8676434011776e090e4cda43e57304be`.
- [`HWP5-nopassword-123456.hwp`](../../../samples/HWP5-nopassword-123456.hwp) — SHA-256 `a34ecb8cde85b6db49c64a954cb7fa5d23b5f49367bc4753c90bfe89a075b50d`.
- [`HWP3-password-123456.pdf`](../../../pdf/HWP3-password-123456.pdf) — SHA-256 `3ced5ad95ad30331e2756b5b34509c1ac91dfe3c72013c8e14f2556ca6bd5776`.

기여자가 제출한 기준 자료: [`h3_hwp5_base_silhouette.tsv`](../../../mydocs/pr/assets/issue_8304/native/h3_hwp5_base_silhouette.tsv). 해당 자료만으로 새 통합 후보의 시각 검증을 통과 처리하지 않는다.

2026-11-01 Native 중간 후보 SHA-256 `a53264efa0f40b8c851f2e6825e984fc9d5666cd9a23e79ad708c2ed1fb71b7a`에서 암호 HWP3 **원본**을 열어 한컴 Print PDF와 전 24쪽을 96 DPI로 대조했다. 시험 fixture 암호는 검토용 래퍼가 표준 입력으로 전달했다. 전쪽 최저 raw는 3쪽 `94.82%`다. [임시 TSV](../../../output/pr-review/planet6897-ordered-20261031/issue8304-current-native-first/issue8304-original/silhouette.tsv), [3쪽 review](../../../output/pr-review/planet6897-ordered-20261031/issue8304-current-p003-review/issue8304-p003/review/review_003.png), [overlay](../../../output/pr-review/planet6897-ordered-20261031/issue8304-current-p003-review/issue8304-p003/overlay/overlay_003.png)를 남겼다. 이후 글꼴 출처 코드가 변경돼 새 후보와 fresh WASM을 다시 확인해야 하며, 이 결과를 최종 검증으로 사용하지 않는다.

3쪽 확대 판독에서는 하단 폴더 설명의 글자 위치와 간격 차이가 눈에 띈다. 90% 통과만으로 해당 영역을 완료로 판정하지 않으며, 같은 입력의 기준 코드·새 후보·독립 Print를 대조해 원인을 확인한다.

같은 시스템 글꼴과 PDF를 사용한 #8397 기준 코드 `99a176b7cb`의 3쪽 raw는 `40.94944%`였다. 위 중간 후보는 `94.82%`로 개선됐지만 잔여 차이의 해결이나 이후 후보의 결과를 뜻하지 않는다. 기준 측정은 결과 산출 후 90% 게이트에서 종료 코드 1을 반환했다([기준 로그](../../../output/pr-review/planet6897-ordered-20261031/issue8304-base-p003.log)).

후속 Native 후보 `b38728ae6740b44deb2a6b5c574110d07c5b44535db34b593284a45276b2a3c7`에서도 같은 암호 원본을 전 24쪽 재출력했다. 각 쪽 PNG SHA-256은 직전 후보와 모두 같고 최저 raw는 3쪽 `94.82%`다([임시 TSV](../../../output/pr-review/planet6897-ordered-20261031/issue8304-hwp5-face-source-native/issue8304-original/silhouette.tsv)). 위 3쪽 이미지는 같은 출력에 재사용할 수 있으나, fresh WASM과 하단 간격의 검토는 남아 있다.

기존 생성 목록으로 최소 글꼴 회귀 12,320건 통과·45건 건너뜀을 확인했다. 생성 목록에서 누락된 #8410 두 모듈의 6건을 목록 재작성 후 같은 환경에서 별도로 실행해 모두 통과했다. 두 실행을 구분하며 한 번의 전체 실행으로 기록하지 않는다.

제외 입력 정리 후 최소 글꼴 환경에서 #8304의 남은 6개 검사가 모두 통과했다. #8303의 3개 검사와 한 번에 실행한 [집중 검사 로그](../../../output/pr-review/planet6897-ordered-20261031/excluded-fixture-focused-minfont.log)에서 두 모듈 합계 9건 통과를 확인했다. 전체 검사의 대체 결과는 아니다.

2026-11-01 고정 후보의 #8389·#8397 전쪽 정밀 대조에서 2,153행 중 Native 12행, WASM 8행이 공개 기준보다 낮았다. 이 수치는 이후 GIF·Canvas 보정 전 결과이며 최종 후보의 재검증 결과가 아니다. #7905 3쪽은 GIF 오른쪽 경계 누락, sample16 35쪽 WASM은 메타파일 내부의 미대응 3픽셀이 확인됐다. #8040 두 원본은 Native·WASM 전쪽이 90% 이상이나 Native 정밀 기준 하락이 남는다. 진행과 근거는 [작업 기록](../../working/task_m100_planet6897_system_fonts_20261031.md)을 따른다. 하락 보정과 전쪽 대조, 각 PR의 독립 Print·Native/fresh WASM 및 직접 판독을 완료해야 한다. #8410·#8412의 새 base 기능도 별도로 보존 검증한다.

원 PR의 최신 head·required check와 통합 후보의 새 검증 결과를 확인하기 전에는 승인이나 merge를 진행하지 않는다.
