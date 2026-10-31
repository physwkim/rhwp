# PR #8338 리뷰 — fix(serializer): HWP↔HWPX 저장의 개체 번호 범주 비트를 numberingType에서 만든다 — 그림 캡션 번호 보존 (#8335)

## 최종 판정

**머지 보류 — 재체리픽 후보의 검증 진행 중.** 최신 base의 기능 보존, 관련 검증과 기존 정밀 TSV 하락 해소를 완료한 뒤 다시 판정한다.

## 접수와 적용

- 원 PR: [#8338](https://github.com/edwardkim/rhwp/pull/8338), 작성자 `planet6897`, reviewer `jangster77`.
- 원 PR head 조회값: `0ac95d8c9599ad98b8f7601faf62243e58814d03`. 조회 시 원 PR head와 마지막 체리픽 원본이 일치한다.
- 마지막 적용 원본: `0ac95d8c9599ad98b8f7601faf62243e58814d03`. 검증 시작 당시 코드 후보: `abea5b028f8f135bdf000ca700e1993ebf0b1973`, 기준 `upstream/devel`: `bc9dfdd057258de59afebeb5121c4bee7398e07f`.
- 관련 이슈: [#8335](https://github.com/edwardkim/rhwp/issues/8335).
- 선행 PR: #8297.

후속 `e1f03ffbb3`은 Python 테스트 준비, `af175315e7`과 `e920cba6b0`은 회귀 검사 주석, `a63bb719de`는 검증 경과를 기록했다. `5d1294811b`은 전체 그림의 잘림 오판을 보정했다. `e1b8ab0f54`, `d6fe30c3ad`, `0897582f6c`, `8c4eca265e`에 Native 진단, #8410 집중 6건 통과와 글꼴 대조를 기록했으며 새 코드의 전쪽 출력 검증이 필요하다.

## 변경 주장과 검토 관점

개체 번호 범주를 numberingType에서 생성해 저장 왕복 후 캡션 번호를 보존한다.
선행 #8297의 범주 파싱과 저장 비트가 같은 계약을 따르는지 확인한다.

## 체리픽과 보정

- 원본 `707465a9958c8ad65821eafb8df19ab0a1771ce4` → 통합 `01858daab44ead057b12f8acf0932fca86ce33de`.
- 원본 `0ac95d8c9599ad98b8f7601faf62243e58814d03` → 통합 `ea52d7741885250b5a41a3cd98ee6eac3808e7fc`.

이 PR의 원본 커밋 적용 중 별도로 기록된 충돌은 없다.

## 검증과 증거

이전 통합 후보의 최소 글꼴 전체 회귀 로그에서 이 PR 관련 다음 모듈의 실행을 확인했다. 새 base 후보의 재실행 결과는 이 값으로 대신하지 않는다.
- [`issue_8335_caption_category_roundtrip.rs`](../../../tests/cases/issue_8335_caption_category_roundtrip.rs): 이전 실행 4건 통과, 0건 실패, 0건 건너뜀.

검토 후보에 포함된 입력 바이트를 Git blob과 대조했다. 아래 자료는 원 PR의 제출 증거이며 새 후보의 실행 결과를 뜻하지 않는다.
- [`hwp3-native-font-program-final-hwp5-20261017-2020.pdf`](../../../pdf/hwp3-native-font-program-final-hwp5-20261017-2020.pdf) — SHA-256 `579648569f6b2ef7d16b4b094525407c9b5b737c5139d8d405d67ee42bd70fca`.
- [`1790387_policy_progress-hwpx-2020.pdf`](../../../pdf/issue4599/1790387_policy_progress-hwpx-2020.pdf) — SHA-256 `4c0d9d0876410766d99f4589916971f97b751a3d71901068637c7e2cc1061f95`.
- [`exam_kor-hwp-2020.pdf`](../../../pdf/exam_kor-hwp-2020.pdf) — SHA-256 `d67a5a0c759dcc8609226f89ef96de5339e3b41c43c2655be3824dcde1732f22`.

기여자가 제출한 기준 자료: [`ek_hwpx_silhouette.tsv`](../../../mydocs/pr/assets/issue_8335/native/ek_hwpx_silhouette.tsv). 해당 자료만으로 새 통합 후보의 시각 검증을 통과 처리하지 않는다.

기존 생성 목록으로 최소 글꼴 회귀 12,320건 통과·45건 건너뜀을 확인했다. 생성 목록에서 누락된 #8410 두 모듈의 6건을 목록 재작성 후 같은 환경에서 별도로 실행해 모두 통과했다. 두 실행을 구분하며 한 번의 전체 실행으로 기록하지 않는다.

2026-11-01 고정 후보의 #8389·#8397 전쪽 정밀 대조에서 2,153행 중 Native 12행, WASM 8행이 공개 기준보다 낮았다. 이 수치는 이후 GIF·Canvas 보정 전 결과이며 최종 후보의 재검증 결과가 아니다. #7905 3쪽은 GIF 오른쪽 경계 누락, sample16 35쪽 WASM은 메타파일 내부의 미대응 3픽셀이 확인됐다. #8040 두 원본은 Native·WASM 전쪽이 90% 이상이나 Native 정밀 기준 하락이 남는다. 진행과 근거는 [작업 기록](../../working/task_m100_planet6897_system_fonts_20261031.md)을 따른다. 하락 보정과 전쪽 대조, 각 PR의 독립 Print·Native/fresh WASM 및 직접 판독을 완료해야 한다. #8410·#8412의 새 base 기능도 별도로 보존 검증한다.

원 PR의 최신 head·required check와 통합 후보의 새 검증 결과를 확인하기 전에는 승인이나 merge를 진행하지 않는다.
