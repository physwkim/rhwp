# PR #8356 리뷰 — fix(cli): export-hwpx --verify가 HWPX 저장의 의도된 정규화를 손실로 세지 않는다 (#8306)

## 최종 판정

**머지 보류 — 재체리픽 후보의 검증 진행 중.** 최신 base의 기능 보존, 관련 검증과 기존 TSV 표시값의 페이지별 무하락 확인을 완료한 뒤 다시 판정한다.

## 접수와 적용

- 리베이스 후 코드 후보: `1f8e948ab443c6f963413e45eb330aab1294ca9f`, 기준 `upstream/devel`: `da39c536613a4bd543dc72b63a556183efc0383f`. 체리픽 대응표에는 새 SHA와 리베이스 전 SHA를 함께 기록했다.
- 원 PR: [#8356](https://github.com/edwardkim/rhwp/pull/8356), 작성자 `planet6897`, reviewer `jangster77`.
- 원 PR head 조회값: `ceb9167e3a6f9bdc564013971039130faf153677`. 조회 시 원 PR head와 마지막 체리픽 원본이 일치한다.
- 마지막 적용 원본: `ceb9167e3a6f9bdc564013971039130faf153677`. 검증 시작 당시 코드 후보: `abea5b028f8f135bdf000ca700e1993ebf0b1973`, 기준 `upstream/devel`: `bc9dfdd057258de59afebeb5121c4bee7398e07f`.
- 관련 이슈: [#8306](https://github.com/edwardkim/rhwp/issues/8306).
- 선행 PR: 별도 누적 선행 PR이 명시되지 않았다.

후속 `e1f03ffbb3`은 Python 테스트 준비, `af175315e7`과 `e920cba6b0`은 회귀 검사 주석, `a63bb719de`는 검증 경과를 기록했다. `5d1294811b`은 전체 그림의 잘림 오판을 보정했다. `e1b8ab0f54`, `d6fe30c3ad`, `0897582f6c`, `8c4eca265e`에 Native 진단, #8410 집중 6건 통과와 글꼴 대조를 기록했으며 새 코드의 전쪽 출력 검증이 필요하다.

## 변경 주장과 검토 관점

HWPX 내보내기 검증에서 저장기의 의도된 정규화를 손실로 잘못 세지 않도록 비교한다.
쪽 번호 접기·합성 줄 정규화와 실제 내용 손실 검출을 함께 확인한다.

## 체리픽과 보정

- 원본 `ceb9167e3a6f9bdc564013971039130faf153677` → 통합 `6db76bcdb9dbfd341534f2ecf142a4f82ecff653` (리베이스 전 `9f098c382a8a1fbe45d50da3bc623abacef87702`).

`src/cli/commands/conversion.rs` 충돌은 이전 보정 당시와 양쪽 Git blob이 동일해 기존 해결본을 재사용했다.

## 검증과 증거

이전 통합 후보의 최소 글꼴 전체 회귀 로그에서 이 PR 관련 다음 모듈의 실행을 확인했다. 새 base 후보의 재실행 결과는 이 값으로 대신하지 않는다.
- [`issue_8306_hwpx_verify_normalization.rs`](../../../tests/cases/issue_8306_hwpx_verify_normalization.rs): 이전 실행 5건 통과, 0건 실패, 0건 건너뜀.

기존 생성 목록으로 최소 글꼴 회귀 12,320건 통과·45건 건너뜀을 확인했다. 생성 목록에서 누락된 #8410 두 모듈의 6건을 목록 재작성 후 같은 환경에서 별도로 실행해 모두 통과했다. 두 실행을 구분하며 한 번의 전체 실행으로 기록하지 않는다.

2026-11-01 고정 후보의 #8389·#8397 전쪽 정밀 대조에서 2,153행 중 Native 12행, WASM 8행이 공개 기준보다 낮았다. 이 수치는 이후 GIF·Canvas 보정 전 결과이며 최종 후보의 재검증 결과가 아니다. #7905 3쪽은 GIF 오른쪽 경계 누락, sample16 35쪽 WASM은 메타파일 내부의 미대응 3픽셀이 확인됐다. #8040 두 원본은 Native·WASM 전쪽이 90% 이상이다. Native 2쪽의 정밀값은 미세하게 낮지만, 사용자 지정 표시값 기준으로는 기존과 현재 모두 100.00%여서 하락이 아니다. 진행과 근거는 [작업 기록](../../working/task_m100_planet6897_system_fonts_20261031.md)을 따른다. 하락 보정과 전쪽 대조, 각 PR의 독립 Print·Native/fresh WASM 및 직접 판독을 완료해야 한다. #8410·#8412의 새 base 기능도 별도로 보존 검증한다.

원 PR의 최신 head·required check와 통합 후보의 새 검증 결과를 확인하기 전에는 승인이나 merge를 진행하지 않는다.
