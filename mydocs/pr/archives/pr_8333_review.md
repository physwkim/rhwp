# PR #8333 리뷰 — fix: HWP↔HWPX 왕복 저장이 원래 계보와 저장 줄 계약을 보존한다 (#8302, #8310)

## 최종 판정

**머지 보류 — 재체리픽 후보의 검증 진행 중.** 최신 base의 기능 보존, 관련 검증과 기존 정밀 TSV 하락 해소를 완료한 뒤 다시 판정한다.

## 접수와 적용

- 원 PR: [#8333](https://github.com/edwardkim/rhwp/pull/8333), 작성자 `planet6897`, reviewer `jangster77`.
- 원 PR head 조회값: `7f74ed09ac40f30160f0530672c504862ec1dd5b`. 조회 시 원 PR head와 마지막 체리픽 원본이 일치한다.
- 마지막 적용 원본: `7f74ed09ac40f30160f0530672c504862ec1dd5b`. 검증 시작 당시 코드 후보: `abea5b028f8f135bdf000ca700e1993ebf0b1973`, 기준 `upstream/devel`: `bc9dfdd057258de59afebeb5121c4bee7398e07f`.
- 관련 이슈: [#8302](https://github.com/edwardkim/rhwp/issues/8302), [#8310](https://github.com/edwardkim/rhwp/issues/8310).
- 선행 PR: 별도 누적 선행 PR이 명시되지 않았다.

후속 `e1f03ffbb3`은 Python 테스트 준비, `af175315e7`과 `e920cba6b0`은 회귀 검사 주석, `a63bb719de`는 검증 경과를 기록했다. `5d1294811b`은 전체 그림의 잘림 오판을 보정했다. `e1b8ab0f54`, `d6fe30c3ad`, `0897582f6c`, `8c4eca265e`에 Native 진단, #8410 집중 6건 통과와 글꼴 대조를 기록했으며 새 코드의 전쪽 출력 검증이 필요하다.

## 변경 주장과 검토 관점

HWP↔HWPX 왕복에서 원래 계보 표식과 저장 줄의 좌표 계약을 보존한다.
형식별 저장·재열기 및 쪽 소속과 본문 보존을 확인한다.

## 체리픽과 보정

- 원본 `a338931aa5093fd6f2cdd3ab656ce8c66298f8b6` → 통합 `76f5a9de8ef7a90f2e1efcf4d807066321c77c68`.
- 원본 `4b78308e2391b3914e5cd63752b96ef27fb3dd25` → 통합 `4e8ac04fd5b2cd165e2ecbe6155275fb4db03620`.
- 원본 `fff29daa1534a179f7eb3bcb4b3d1de884aea32e` → 통합 `c1ca82f2233d18142ef85d6db55939d0dbce0c21`.
- 원본 `7f74ed09ac40f30160f0530672c504862ec1dd5b` → 통합 `ec9481649c6a6a4dc382bc07b32cd57e0bf2ef1d`.

이 PR의 원본 커밋 적용 중 별도로 기록된 충돌은 없다.

## 검증과 증거

이전 통합 후보의 최소 글꼴 전체 회귀 로그에서 이 PR 관련 다음 모듈의 실행을 확인했다. 새 base 후보의 재실행 결과는 이 값으로 대신하지 않는다.
- [`issue_8302_roundtrip_lineage.rs`](../../../tests/cases/issue_8302_roundtrip_lineage.rs): 이전 실행 3건 통과, 0건 실패, 0건 건너뜀.
- [`issue_8310_hwp_save_source_vpos.rs`](../../../tests/cases/issue_8310_hwp_save_source_vpos.rs): 이전 실행 5건 통과, 0건 실패, 0건 건너뜀.

검토 후보에 포함된 입력 바이트를 Git blob과 대조했다. 아래 자료는 원 PR의 제출 증거이며 새 후보의 실행 결과를 뜻하지 않는다.
- [`2912735_court_report_form-2020.pdf`](../../../pdf/2912735_court_report_form-2020.pdf) — SHA-256 `039f83dda89a7ac1fe3a162b39fdee0afbc43f4cc98af0c48a1223eed38d27a8`.
- [`worklife_balance_index_156607916-2020.pdf`](../../../pdf/planet-review-20260917/worklife_balance_index_156607916-2020.pdf) — SHA-256 `0cc7229c1d8da5ef425ef76f3ba143db17171a71449bc4e341579a18a07a47ec`.

기여자가 제출한 기준 자료: [`2912735_orig_silhouette.tsv`](../../../mydocs/pr/assets/issue_8302_8310/native/2912735_orig_silhouette.tsv). 해당 자료만으로 새 통합 후보의 시각 검증을 통과 처리하지 않는다.

기존 생성 목록으로 최소 글꼴 회귀 12,320건 통과·45건 건너뜀을 확인했다. 생성 목록에서 누락된 #8410 두 모듈의 6건을 목록 재작성 후 같은 환경에서 별도로 실행해 모두 통과했다. 두 실행을 구분하며 한 번의 전체 실행으로 기록하지 않는다.

2026-11-01 고정 후보의 #8389·#8397 전쪽 정밀 대조에서 2,153행 중 Native 12행, WASM 8행이 공개 기준보다 낮았다. 이 수치는 이후 GIF·Canvas 보정 전 결과이며 최종 후보의 재검증 결과가 아니다. #7905 3쪽은 GIF 오른쪽 경계 누락, sample16 35쪽 WASM은 메타파일 내부의 미대응 3픽셀이 확인됐다. #8040 두 원본은 Native·WASM 전쪽이 90% 이상이나 Native 정밀 기준 하락이 남는다. 진행과 근거는 [작업 기록](../../working/task_m100_planet6897_system_fonts_20261031.md)을 따른다. 하락 보정과 전쪽 대조, 각 PR의 독립 Print·Native/fresh WASM 및 직접 판독을 완료해야 한다. #8410·#8412의 새 base 기능도 별도로 보존 검증한다.

원 PR의 최신 head·required check와 통합 후보의 새 검증 결과를 확인하기 전에는 승인이나 merge를 진행하지 않는다.
