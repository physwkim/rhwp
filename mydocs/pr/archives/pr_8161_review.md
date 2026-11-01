# PR #8161 리뷰 — fix(#8079): 굵은 글자 전진폭은 보통 면 값 — 저장 줄 포함

## 최종 판정

**머지 보류 — 재체리픽 후보의 검증 진행 중.** 최신 base의 기능 보존, 관련 검증과 기존 TSV 표시값의 페이지별 무하락 확인을 완료한 뒤 다시 판정한다.

## 접수와 적용

- 리베이스 후 코드 후보: `1f8e948ab443c6f963413e45eb330aab1294ca9f`, 기준 `upstream/devel`: `da39c536613a4bd543dc72b63a556183efc0383f`. 체리픽 대응표에는 새 SHA와 리베이스 전 SHA를 함께 기록했다.
- 원 PR: [#8161](https://github.com/edwardkim/rhwp/pull/8161), 작성자 `planet6897`, reviewer `jangster77`.
- 원 PR head 조회값: `bd786fc5fefb94126ac146411f603c9cbcf336a6`. 현재 원 PR head는 기여자 마지막 커밋 뒤의 devel merge이며 merge commit은 제외했다.
- 마지막 적용 원본: `431c6a7b508305673e954b796eaff59fd295715e`. 검증 시작 당시 코드 후보: `abea5b028f8f135bdf000ca700e1993ebf0b1973`, 기준 `upstream/devel`: `bc9dfdd057258de59afebeb5121c4bee7398e07f`.
- 관련 이슈: [#8079](https://github.com/edwardkim/rhwp/issues/8079).
- 선행 PR: 별도 누적 선행 PR이 명시되지 않았다.

후속 `e1f03ffbb3`은 Python 테스트 준비, `af175315e7`과 `e920cba6b0`은 회귀 검사 주석, `a63bb719de`는 검증 경과를 기록했다. `5d1294811b`은 전체 그림의 잘림 오판을 보정했다. `e1b8ab0f54`, `d6fe30c3ad`, `0897582f6c`, `8c4eca265e`에 Native 진단, #8410 집중 6건 통과와 글꼴 대조를 기록했으며 새 코드의 전쪽 출력 검증이 필요하다.

## 변경 주장과 검토 관점

굵은 TrueType의 잉크는 Bold face를 쓰고 전진폭은 같은 family의 보통 face를 사용하며 HFT는 제외한다. 저장 줄과 새 줄을 함께 다룬다.
저장 줄과 재조판의 측정·paint 소비가 독립 Print와 일치하는지 확인한다.

## 체리픽과 보정

- 원본 `df9d84afad9c965579465e9c95362f4bd0a7a67e` → 통합 `fe2f1d422453095a7077625a353691272bf58b8a` (리베이스 전 `859a62eebb6ec03ea8a9fc0e2848d0956bb8f18a`).
- 원본 `97a95fdb89dc60dc395fd6cf9a2a6899f2729a74` → 통합 `fa15a388578323352a9735bdea7212ecd34be948` (리베이스 전 `9b59df521c9c427dfb9e8ea352d2337213cc3bb3`).
- 원본 `5476c862d9d02f5795e73d52dc137099de036bf7` → 통합 `0f3b844b66a1ba0b9209679051b186b91e86451d` (리베이스 전 `d187d6762df9e77e5aba15b6558e2a540914852e`).
- 원본 `fe19ddb8729eaedc3ed8010c24d0a1cf51bd394f` → 통합 `ba452bf4fed40bfc8e56b02667eb996bc77553ce` (리베이스 전 `4e084badab6e26b1bde142c1ab4bea635d08b230`).
- 원본 `32148bff81330f3a6dadcab850c14d8c9dc0f441` → 통합 `391b5c0d904c0faadca067f91dc5c81571c3016c` (리베이스 전 `2dc6adff5d258785b8701ed30e84cf2749ec82ab`).
- 원본 `431c6a7b508305673e954b796eaff59fd295715e` → 통합 `1a5c8fb0dc71a7be794dac14b314c60f52e360be` (리베이스 전 `03d70548e14677ac9911f60664dfb5331bfacb11`).

`src/renderer/mod.rs`의 `LatinSlotMetrics` 복제 필드 충돌에서 새 base의 `native_otf_metrics`와 PR의 `source_font_is_hft`를 함께 보존했다.

## 검증과 증거

이전 통합 후보의 최소 글꼴 전체 회귀 로그에서 이 PR 관련 다음 모듈의 실행을 확인했다. 새 base 후보의 재실행 결과는 이 값으로 대신하지 않는다.
- [`hci_poppy_source_metric_contract.rs`](../../../tests/cases/hci_poppy_source_metric_contract.rs): 이전 실행 2건 통과, 0건 실패, 0건 건너뜀.
- [`issue_8079_bold_regular_advance.rs`](../../../tests/cases/issue_8079_bold_regular_advance.rs): 이전 실행 4건 통과, 0건 실패, 0건 건너뜀.

현재 코드에서 `issue_8079_bold_regular_advance` 4건을 최소 글꼴 환경으로 다시 실행해 모두 통과했다([집중 검사 로그](../../../output/pr-review/planet6897-ordered-20261031/font-slot-focused-minfont-after-commit.log)). 같은 실행은 #8136·#8315를 포함한 10건이며, #8161의 나머지 시각·회귀 검증 전체를 대신하지 않는다.

메인터너 보정 `6be28314c5`는 실제 원본 프로그램의 글리프 소유가 남아 있지만 글꼴 유형이 미확정인 경우, 표시 대체 글꼴의 보통 전진폭을 원본의 굵은 전진폭으로 잘못 사용하지 않도록 공통 측정 경계를 수정한다. 명시 TTF와 원본 프로그램 소유가 없는 경우의 기존 #8079 계약은 유지한다. 원본 HCI Poppy의 독립 프로그램 폭과 명시 TTF 반례를 포함한 최소 글꼴 집중 회귀 32건은 통과했다([로그](../../../output/pr-review/planet6897-ordered-20261031/source-bold-boundary-focused-minfont.log)). 이 결과는 전체 회귀 또는 Native·fresh WASM 전쪽 시각 검증을 대신하지 않는다.

검토 후보에 포함된 입력 바이트를 Git blob과 대조했다. 아래 자료는 원 PR의 제출 증거이며 새 후보의 실행 결과를 뜻하지 않는다.
- [`2026_oss_rst.hwp`](../../../samples/2026_oss_rst.hwp) — SHA-256 `b3a43578fc441d9668d06cd55c0cd267332ecb087e47ada4c5ac456af2afc78b`.
- [`2026_oss_rst.pdf`](../../../pdf/hwpx/2026_oss_rst.pdf) — SHA-256 `bec53a601cc7714a40ca340d26f971d1ab49eeb355682fc9db7b15cd5e04c86e`.
- [`1613000-202200037-air-traffic-controller-cbta.hwp`](../../../samples/issue6764/1613000-202200037-air-traffic-controller-cbta.hwp) — SHA-256 `8ef9de3f35690bf9d7994527f77cb02d4a4fcff447c219a78fbc2855d64be6e7`.

기여자가 제출한 기준 자료: [`README.md`](../../../mydocs/pr/assets/issue8079_bold_regular_advance/README.md). 해당 자료만으로 새 통합 후보의 시각 검증을 통과 처리하지 않는다.

기존 생성 목록으로 최소 글꼴 회귀 12,320건 통과·45건 건너뜀을 확인했다. 생성 목록에서 누락된 #8410 두 모듈의 6건을 목록 재작성 후 같은 환경에서 별도로 실행해 모두 통과했다. 두 실행을 구분하며 한 번의 전체 실행으로 기록하지 않는다.

2026-11-01 고정 후보의 #8389·#8397 전쪽 정밀 대조에서 2,153행 중 Native 12행, WASM 8행이 공개 기준보다 낮았다. 이 수치는 이후 GIF·Canvas 보정 전 결과이며 최종 후보의 재검증 결과가 아니다. #7905 3쪽은 GIF 오른쪽 경계 누락, sample16 35쪽 WASM은 메타파일 내부의 미대응 3픽셀이 확인됐다. #8040 두 원본은 Native·WASM 전쪽이 90% 이상이다. Native 2쪽의 정밀값은 미세하게 낮지만, 사용자 지정 표시값 기준으로는 기존과 현재 모두 100.00%여서 하락이 아니다. 진행과 근거는 [작업 기록](../../working/task_m100_planet6897_system_fonts_20261031.md)을 따른다. 하락 보정과 전쪽 대조, 각 PR의 독립 Print·Native/fresh WASM 및 직접 판독을 완료해야 한다. #8410·#8412의 새 base 기능도 별도로 보존 검증한다.

원 PR의 최신 head·required check와 통합 후보의 새 검증 결과를 확인하기 전에는 승인이나 merge를 진행하지 않는다.
