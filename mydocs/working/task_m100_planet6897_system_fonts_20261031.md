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

## 정밀 기준 하락의 원인 분리

`sample16` 18쪽의 base와 후보를 같은 시스템 글꼴, 같은 명시 글꼴 환경과 Print로
재측정하니 Native PNG SHA-256과 미대응·합집합 정수가 모두 같았다. 이전 환경이 다른
base 출력과의 차이를 코드 회귀로 단정하지 않는다. 다만 이 시스템 글꼴 조건의 결과는
#8397에 보존된 정밀 기준보다 낮다.

당시 사용한 글꼴 목록에는 `HCI Poppy`의 완성된 기호 포함 프로그램이 명시됐고, 현재
시스템에는 같은 family의 다른 프로그램도 등록돼 있다. 원래 프로그램을 우선 공급하는
진단은 다른 시스템 글꼴을 노출한 상태에서 미대응/합집합을 `15/206244`에서
`15/206246`으로 바꿔 기존 정밀 기준을 복원했다. 이는 특정 입력에 대한 진단이며
자동 선택의 공통 규칙이나 다른 페이지의 검증을 대신하지 않는다. 입력·환경·로그는
`output/pr-review/planet6897-ordered-20261031/sample16-p18-full-system-poppy-diagnostic/`에
있다. 검증 글꼴 프로그램 자체는 저장소에 추가하지 않는다.

편람 145쪽은 원본에 지정된 정부상징 글꼴이 현재 시스템에 설치돼 있고, 기존 Print는
대체 한컴바탕을 사용한다. #8389의 실제 공급자 계약을 따라 원본 설치 여부를 무시한
문서별 강제 치환은 적용하지 않았다. #8040 기호 슬롯은 원 PR의 독립 Print 계약과
함께 별도 분석 중이다. Native 12행과 WASM 8행의 기존 정밀 기준 하락을 해소하기 전에는
검토를 완료하지 않는다.

같은 진단을 HWP 2024 원본 18쪽에도 실행해 `15/206244`에서 `15/206246`으로
복구했다. 이 두 결과는 프로그램 선택에 따른 재현이며 제품의 기본 자동 선택을
수정한 결과가 아니다. 해당 시스템에는 같은 family의 서로 다른 CFF 프로그램이
존재한다. 기존 glyph 목록의 합집합 크기만으로 프로그램의 원본 신원을 추측하지 않는다.

공통 공급 환경의 `programId`는 식별 및 환경 fingerprint에 사용되지만 스타일 해소는
현재 최종 family를 반환한다. 다만 이번 sample16 환경 JSON은 Poppy programId를
포함하지 않아 이 반환 경계가 이번 결과의 직접 원인이라고 단정하지 않는다.
서로 다른 동일 family 프로그램의 선택과 backend 전달을 독립적으로 검증해야 한다.

## 최신 base 재적용과 회귀 수집 점검

`bc9dfdd057258de59afebeb5121c4bee7398e07f` 위에 체리픽을 다시 적용한 후, 기존
생성 목록을 갱신하지 않고 최소 글꼴 전체 회귀를 실행했다. 이 목록에서는 12,320건이
통과하고 45건이 건너뛰어졌지만 새 base #8410의 PDF 패턴·문자 획 검사 두 모듈이
빠져 있었다. 이 결과만으로 최신 source의 전체 검사가 완료됐다고 판단하지 않는다.

`node scripts/rust-test-suite-manifest.mjs --prepare`와 고정 base `--check`를 실행해
두 모듈을 수집 목록에 포함했다. 사용자 요청에 따라 이미 통과한 검사를 다시 반복하지 않고
같은 최소 글꼴 환경에서 누락된 6건을 따로 실행했으며 모두 통과했다. 전체 목록과 누락분의
결과는 각각 독립된 실행으로 기록한다. 생성기 계약 검사 23건도 통과했다.
첫 번째 실행과 집중 검사 사이 제품 Rust 코드는 바뀌지 않았고, 회귀 검사 주석만 변경했다.

Visual Sweep의 재개 기능은 입력 해시를 확인하므로 기존 raw gate 단위 검사의 가상 경로가
존재하지 않아 실패했다. 테스트가 입력 파일과 해시를 준비하도록 보정했으며 90% 경계와
반올림 전 판정 기대값은 유지했다. Visual Sweep Python 검사 210건과 글꼴 공급 Node 검사
28건이 통과했다. 해당 실행 로그는 검토용 `output/pr-review/planet6897-ordered-20261031/`
아래에 두며 Git에는 포함하지 않는다.

리베이스 전 후보의 TSV 하락 12행(Native)·8행(WASM)은 새 후보의 결과가 아니다. 새 base의
제품 코드로 Native와 fresh WASM을 빌드하고, 같은 시스템 글꼴과 독립 Print를 사용해 새
출력 경로에서 다시 측정해야 한다. 빌드 head와 바이너리 해시가 맞지 않으면 재측정
스크립트가 시작하지 않도록 준비했다.

## #8040 전체 그림의 자르기 오판 원인과 보정 계획

#8040 두 원본의 2쪽에서 이전 후보와 재적용 후보의 Native PNG는 각각 동일한
`(558, 908)–(947, 988)` 그림 영역에서만 6,912픽셀 다르다. 두 실행의 해당 쪽 render tree와
독립 Print 래스터 해시는 각각 일치한다. 원본 그림의 저장 crop은 `(0, 0, 66180, 13980)`,
전체 기준 `imgDim`은 `(66180, 13980)`, 디코딩 BMP는 `882×186`이다. crop은 원본 단위에서
전체 범위와 정확히 같다.

`compute_image_crop_src`에서 `66180 / (66180 / 882)`가 부동소수점으로
`881.9999999999999`가 된다. 새 base #8410의 `image_source_is_full`은 변환 결과의 정확한
동일성으로 판정하므로 전체 그림을 잘린 그림으로 오인한다. 실제 SVG도 해당 그림에
`viewBox="0 0 881.9999999999999 186"`을 생성하며, 이전 base SVG는 중첩 crop viewport가
없다. 이전 후보와 새 후보의 미대응 픽셀은 각 1개지만 합집합은 각각 202315와 202439다.
이 현상만으로 #8040의 모든 TSV 하락을 설명하지는 않는다.

보정은 공통 `compute_image_crop_src`에서 유효한 crop 기준 크기와 원본 crop이 원점부터
전체 범위까지 정확히 같을 때, 변환 전의 전체 소유 관계에 따라 디코딩 이미지 크기를 직접
반환한다. 다른 crop은 기존 계산과 #8410의 정확 판정을 유지한다. 원본의 실제 한 픽셀
자르기와 2픽셀 타일의 반쪽 선택은 전체 그림이 아니므로 그대로 잘려야 한다. 이 조건은
문서명이나 출력 점수가 아니라 저장 좌표의 동등성을 이용한다.

검증은 보정 전 실패하는 전체 그림 및 실제 잘림 반례, #8410 패턴·저장 왕복 6건,
#8040 동일 입력의 독립 Print 대조와 Native/fresh WASM 기준 TSV를 사용한다. 보정 전
출력과 로그는 `output/pr-review/planet6897-ordered-20261031/replay-bc9-new-sweep/`에 보존한다.

수정 전후의 임시 계약 진단은 전체 그림의 정확한 원점·끝을 수정 전 실패/수정 후
통과로 재현했다. 실제 1원본 단위 잘림과 #8410의 2픽셀 이미지 반쪽 선택은 양쪽에서
통과했다. 이는 공통 함수의 진단이며 정식 회귀나 Native/fresh WASM 증거를 대체하지 않는다.
수정 후 `release-test` Native 진단 SVG는 해당 2쪽의 불필요한 중첩 crop viewport를
제거했다. 같은 문서의 4쪽 Native 재측정은 1·3·4쪽이 수정 전과 같고, 2쪽 raw는
`99.999505711504%`다. #8397 기준 `99.999506311804%`보다 약 `0.000000600300%p`
낮다. 나머지 세 쪽도 기존 기준보다 낮으므로 이 보정만으로 무하락을 달성하지 못했다.
진단 로그와 전쪽 TSV는 `output/pr-review/planet6897-ordered-20261031/crop-fix-8040-native/`에
있으며, 최종 release Native·fresh WASM 재측정은 아직 필요하다.

#8040 기호 대체를 재검토하며 현재 시스템의 `HY Sinmyeongjo`와 U+3260을 Fontconfig에
질의했다. 기본 일치 결과는 Noto Sans CJK 계열부터 시작하며, 한컴 Print에 임베드된
대응 윤곽이 대다수 일치한 Gulim과 동일한 선택을 보장하지 않았다. 따라서 시스템의 첫
대체 face나 특정 문서에 Gulim을 강제하는 방식은 공통 해결 근거가 되지 않는다. 선택 결과와
실제 글리프 공급은 추가 추적이 필요하다.

공통 보정 후 최소 글꼴 환경에서 `native-skia`를 켜고 #8410 이미지 패턴·문자 획의
기존 두 모듈을 선택해 실행한 결과 6건 모두 통과했다. 테스트 모음 전체가 아니라 선택한
6건의 결과다. 로그는 `output/pr-review/planet6897-ordered-20261031/crop-fix-8410-native-skia-tests.log`다.

#8040 수정 후 2쪽의 raw 미대응 1픽셀은 이미지가 아니라 본문의 `<1` 글자 중 숫자
`1` 부근에서 발생했다. 후보 PNG는 `(580, 621)`이 회색이고 같은 독립 Print 위치는
흰색이며 2px 이웃 판정에서도 대응 내용이 없다. Render tree에는 해당 위치를 포함하는
`<1 ` 텍스트 run이 있다. 같은 문서를 같은 시스템 글꼴로 다시 출력한 진단 도구의 PNG와
Visual Sweep PNG SHA-256은 모두 `3f241958ebcb452a099b85b55d8b2ad0d37036cc1e2f21ebe9b336433c7a55ea`다.
글꼴 trace에서 이 형태의 문자 쌍은 원문 HFT `한양신명조`, Native OTF 메트릭,
실제 선택 family `HY Sinmyeongjo`로 기록된다. family만으로 파일 단위 글꼴 프로그램이나
해당 1픽셀의 근본 원인을 확정하지 않는다. 해당 PNG·trace와 로그는
`output/pr-review/planet6897-ordered-20261031/crop-fix-8040-p002-trace.*`에 있다.
독립 PDF 2쪽의 해당 CID 글꼴은 문서에 보이는 이름과 달리 FontDescriptor의
`/FontName`이 `/INPILL+Gulim`이다. `/CIDToGIDMap /Identity`와 ToUnicode로 확인한
숫자 `1` 글리프 38의 윤곽은 로컬 Gulim의 `1`과 같고, 두 글꼴의 unitsPerEm은 모두
1024다. 같은 방법으로 비교 가능한 F4 매핑 43개도 로컬 Gulim과 모두 일치했다.
이는 해당 PDF에 사용된 실제 프로그램의 증거다. RHWP가 원문 HFT와 별개로 대체할 조건이나
해당 시스템에서 실제 선택한 파일의 해시를 입증하지는 않으므로 강제 치환하지 않았다.
시스템 글꼴을 그대로 노출한 분석에서 원문 한양신명조의 표시 치환만 Gulim으로 바꾸고
원본 메트릭을 유지해 같은 네 쪽을 재출력했다. 1·2쪽은 raw 미대응이 0이 됐지만
3쪽 `99.791269977850%`, 4쪽 `99.699431809542%`로 #8397의 해당 기준보다 각각
`0.021869064446%p`, `0.018562864253%p` 낮아졌다. 따라서 family 전체를 Gulim으로
바꾸는 것은 채택하지 않는다. 서로 다른 글리프·언어 슬롯과 원본 프로그램의 대응을 더
확인해야 한다. 이 출력은 보정된 제품의 정상 결과가 아니다.

동일한 보정 후 두 번째 입력 `exam-social-insert-delete.hwp`도 Native 진단 4쪽을
독립 Print와 비교했다. raw는 각 `99.959017156908%`, `99.999505606423%`,
`99.813013588419%`, `99.717936122913%`이며, #8397 정밀 기준보다 각각
`0.000075822890%p`, `0.000000590802%p`, `0.000125453877%p`, `0.000058550882%p`
낮다. 단위 테스트 성공이나 90% 초과를 무하락의 대체 근거로 쓰지 않는다.

공통 보정 후 #7905의 Native 진단 빌드는 6쪽 raw가 각각 `99.980577882690%`,
`98.864278150371%`, `99.803773584906%`, `100%`, `100%`, `100%`다. 각 쪽 모두
#8397 정밀 기준보다 높으며 최저는 2쪽이다. 2쪽 PNG SHA-256은 보정 전과 모두
`94b04df710b3995822817311db3683dfb75861177a029fa04f9aab3446612f72`로 같아 기존
review·overlay를 재사용해 직접 확인했다. 본문에 보이는 글꼴·미세 위치 차이는 별도
검토 대상으로 남는다. 이 결과는 Native 진단 프로필이며 fresh WASM이나 최종 release
산출물의 전쪽 검증을 대신하지 않는다.

#7905 2쪽 Native 진단 trace를 같은 입력과 자동 글꼴 환경으로 다시 렌더했으며 PNG
SHA-256이 Visual Sweep 출력과 동일함을 확인했다. 사용자 화면에서 차이가 보이는 조문
제목의 원문 글꼴은 HFT `한양견고딕`, Native trace의 선택 family는 `HY Gyeongothic`,
메트릭 경로는 `nativeOtf`다. 독립 Print의 해당 제목은 Type 3 글꼴이며 PDF 텍스트
API는 family 이름을 제공하지 않는다. 좌표 대조에서 `정부출연연구기관의 범위` 첫 글자의
Print bbox 시작은 약 `(114.56, 591.52)` CSSpx이고 Native render tree run 시작은
약 `(114.7, 591.8)` CSSpx이다. 두 bbox는 산출 계층이 달라 이 수치만으로 차이의
원인을 특정하거나 고정 px 회귀로 삼지 않는다. HFT 원본 윤곽과 실제 선택 프로그램의
대응을 추가 검증해야 한다. 진단 trace는
`output/pr-review/planet6897-ordered-20261031/crop-fix-7905-p002-trace.json`이다.

PDF 텍스트 연산자 확인 결과 #7905의 `정부출연연구기관의` 제목은 Type 3 `/T2`를
사용하며, 본문의 같은 단어는 `/T1`을 사용한다. 시스템 Skia의 `HY Gyeongothic`
후보 목록에서 보통체 한글 `정`을 공급하는 후보는 `HYGyeongothic-HGGGT`다.
해당 로컬 `HGGGT.otf`의 SHA-256은
`da42e4819214f960b3561d4acf51571ad0c887f258e3dc034f5c2fae5c842a68`이다. 현재 trace는
family까지 관찰하고 파일 신원을 직접 기록하지 않으므로 실제 사용 파일이라는 결론은
후보 조회와 코드 순서에 근거한 추론이다. PDF `/T2`의 `정`은 `/CharProcs /HFT224`이며
별도 원본 곡선 비교가 필요하다. 로컬 글꼴 파일은 저장소에 포함하지 않는다.
PDF 텍스트 연산자로 해당 제목의 `/T2` 사용을 확인한 뒤, `정`의 Type 3
`/HFT224` 경로와 로컬 `HGGGT.otf`의 같은 글리프를 각각 1,000단위 SVG 경로로 그려
동일한 `rsvg-convert` 경로로 래스터화했다. 알파 128 기준 내용 합집합 368,798픽셀 중
926픽셀이 다르다. 이는 큰 정규화 격자에서의 윤곽 진단이며 실제 쪽의 raw 점수나
최종 선택 글꼴 파일의 직접 관찰을 뜻하지 않는다. 두 윤곽이 바이트 단위로 같다는
가정은 사용할 수 없다.

## 전체 그림 회귀의 시각 근거

`5d1294811b`의 제품 코드로 fresh WASM을 빌드하고 #8040의 삽입·삭제 원본을 각각
한컴 2020 Print와 전 4쪽 대조했다. 두 원본 모두 Native 최저 `99.717936122913%`,
WASM 최저 `99.708907814252%`다. WASM의 여덟 쪽은 각각 #8389의 기존 정밀 TSV보다
높다. Native는 이전에 확인한 여덟 쪽의 정밀 하락이 여전히 남아 있으므로 제품 전체의
무하락 판정은 아니다.

두 원본의 2쪽을 각각 Native와 WASM review·overlay로 확인하고, 실제 crop viewport의
소유 관계를 검사하는 글꼴 비의존 회귀를 추가했다. 값과 원본·PDF 및 직접 확인 자료는
[`integration_20261031_full_image_crop`](../pr/assets/integration_20261031_full_image_crop/README.md)에
기록했다. 기존 #8410의 실제 반쪽 타일·저장 왕복 검사 여섯 건은 공통 보정 후 통과했다.

#8040이 저장한 문서 슬롯별 `font-environment.json`을 현재 제품에 적용하되 시스템 글꼴
전체를 노출해 삽입 원본을 재측정했다. 1~4쪽은 각각 `99.954648526077%`,
`99.435036482167%`, `99.712504159688%`, `99.077296240378%`다. 이 설정을 그대로
재사용하면 현재 자동 선택의 전쪽보다 낮으며 저장된 #8389 정밀 기준도 복원하지 못한다.
이 실험은 문서별 글꼴을 제품에 강제하는 근거가 아니며,
`output/pr-review/planet6897-ordered-20261031/crop-fix-8040-recorded-env-system-native/`에
명시적 환경의 결과를 보존한다.

sample16 HWPX 18쪽은 이전의 Poppy 명시 공급 진단 PNG와 현재 자동 선택 PNG를 같은
232 문턱으로 비교했다. 서로 다른 내용 마스크는 두 픽셀이며 `(684,1004)`와
`(685,1004)`에 있다. 두 픽셀은 render tree의 `UNIX AP` run 영역에 속한다. 설치된
두 HCI Poppy OTF의 `P` 글리프는 동일하지 않으며 각각 advance 604/1000과 603/1000이다.
이 관찰은 명시 공급의 두 픽셀 복구와 일치하지만, 현재 각 픽셀의 실제 선택 프로그램을
파일 해시까지 추적한 것은 아니므로 자동 선택 변경의 근거로 단정하지 않는다.
시스템 Skia의 같은 family·보통체 후보는 `HCIPoppy-HMEPO` 다음에
`HCIPoppy-IndependentASCII-Regular-CompleteGraphics-CompleteSymbols` 순서이며 두
프로그램 모두 영문 글리프를 제공한다. 글리프 선택 코드와 후보 순서는 첫 프로그램을
선택한다는 추론을 뒷받침하지만, 해당 페이지 paint의 파일 신원을 직접 기록한 결과는
아니다. 후보 조회 로그는 `poppy-system-faces.log`다.
독립 2024 Print의 18쪽에서 해당 `UNIX AP`는 Type 3 `/T3`를 사용하고 `P`는
`/HFT102`다. PDF 곡선 명령과 두 로컬 OTF의 RecordingPen 결과를 좌표 단위로
비교한 결과, 원본 보존 프로그램은 18개 윤곽 명령과 advance 603/1000이 모두
일치한다. 다른 설치 프로그램은 20개 명령과 advance 604/1000으로 일치하지 않는다.
이는 해당 Print 글리프의 원본 프로그램 근거이며 실제 자동 paint가 선택한 파일의
직접 관찰은 여전히 별도다.
동일한 현재 제품 실행 파일·환경·18쪽에서 시스템 글꼴은 모두 유지한 채 두 프로그램을
각각 명시 공급했다. 자동 선택과 `HMEPO.otf` 명시 공급의 페이지 PNG SHA-256은 모두
`9e69c6cccfaf1e34b51f3d452d1bfe3dc6a283949afedd6c6d80fe8498207373`,
미대응/합집합은 `15/206244`로 같다. Print와 `P` 윤곽이 같은 프로그램을 명시 공급하면
PNG SHA-256 `bd44db9662c0e2f8f486a16576162fcde8d9ffc83129866b774e3dfa1f2ef82f`,
`15/206246`이 되어 기존 정밀 기준이 복원된다. PNG 동일성은 그 프로그램이 실제 선택된
파일이라는 직접 추적은 아니며, 공통 우선순위 보정을 단독으로 정당화하지 않는다.
기존 독립 근거의 HFT ASCII 95개 폭을 PDF Type 3의 정수 변환
`floor(HFT폭 × 1000 / 512)`과 대조하면 원본 보존 OTF는 95개가 일치한다.
`HMEPO.otf`는 70개만 일치하고 나머지 25개는 각 1단위 크다. 폭의 일치는 윤곽이나
프로그램 전체의 동일성을 단독으로 증명하지 않으므로, 다른 family·굵기·backend와
명시 프로그램 선택 계약을 확인하기 전에 순위 규칙으로 일반화하지 않는다.
실제 Native 선택 trace용 `native-skia` 진단 빌드는 종료 코드 143으로 중단됐으며
완료로 기록하지 않는다. 원인은 로그만으로 확인되지 않았다. 같은 시각 중단된 전쪽
WASM 비교는 고정 실행 파일과 체크포인트로 재개했다.

## #7905 3쪽 그림 가장자리

공통 보정 이후 fresh WASM 3쪽은 `99.805284771022%`로 #8389 기준
`99.823427543040%`보다 `0.018142772018%p` 낮다. Native와 WASM 모두 한컴 Print에만
있는 미대응 156픽셀이 x=677의 같은 세로선이며, 출력 상단 그림의 오른쪽 경계다.
HWPX에 포함된 GIF는 581×161픽셀이고 양쪽 끝 열이 모두 어둡다. 그림의 `orgSz`는
5810×1610, `curSz`는 44156×12236, `imgClip`은 (0, 0, 44200, 12300), `imgDim`은
없다. 현재 공통 crop 폴백은 원본 가운데 578.5528455284554×161픽셀만 선택하므로
오른쪽 끝 열을 그리지 않는다. 색상 보정이나 글꼴 선택 문제가 아니다.
독립 PDF의 해당 `/Im9`도 581×161픽셀이다. PDF 이미지 변환 폭은 441.363pt이고
클립 폭은 441.243pt로 오른쪽이 0.12pt만 잘린다. 따라서 기준 출력이 수학적으로
완전한 전체 창이라고 단정하지 않고, 실제 오른쪽 끝선이 표시된다는 사실을 계약으로 쓴다.

단순히 `imgClip`이 `curSz`를 포함한다는 이유로 전체 그림을 선택하면 안 된다.
저장소 HWPX에서 같은 조건을 충족하면서 잘림 끝점이 `orgSz`보다 작은 사례가 8개
발견됐다. 현재 크기·원본 크기·변환 행렬과 crop 좌표계의 관계를 독립 Print 및
실제 부분 잘림 반례로 확인해야 한다. 기존 #3239의 비-96dpi 전체 그림과 #7015의
부분 crop 계약도 공통 경로에서 유지해야 한다. 관련 PR은 #3241이며, #8410의 반쪽
타일 검사도 영향 대조군이다. 3쪽 [review](../../output/pr-review/planet6897-ordered-20261031/crop-fix-7905-p003-wasm-review/issue7905/review/review_003.png)와
[overlay](../../output/pr-review/planet6897-ordered-20261031/crop-fix-7905-p003-wasm-review/issue7905/overlay/overlay_003.png)는 임시 진단 결과다.

한컴 2020 MCP(11.0.0.9136)로 이 HWPX를 HWP로 저장했을 때도 원본 크기·현재 크기·
7.6배 행렬·`imgClip` 값이 그대로 보존됐다. 별도 한컴 저장 결과이지 Print PDF를
재산출한 것은 아니다. 같은 엔진이 #7015 HWP를 HWPX로 저장한 결과는 `imgDim=0`,
`imgClip=(0,20745,88560,45453)`으로 실제 위쪽 잘림을 유지한다. #7688의 실제
아래쪽 잘림을 저장한 HWPX는 `imgDim=47940×5280`, `imgClip=(0,0,47940,4366)`이다.
명시 `imgDim`의 부분 선택과 비영 시작점의 부분 선택은 보정 후에도 유지해야 한다.
MCP 변환 입력과 상태·출력은 각각 `issue7015-hwpx-mcp-*`, `issue7688-hwpx-mcp-*`,
`issue7905-hwp-mcp-*` 이름으로 `output/pr-review/planet6897-ordered-20261031/`에 둔다.

별도 #6465 원본 `samples/issue6465/press_release_footer_logos.hwpx`의 13쪽 그림
`image3`은 JPEG 94×33픽셀, `orgSz=9480×3300`, `curSz=6860×2388`,
`imgClip=(0,0,7080,2460)`이며 `imgDim`이 없다. 기존 한컴 Print PDF
`pdf/pr6481-visual/pr6481-issue6465-press-release-footer-logos-2020.pdf` 13쪽의
`/Im3`는 94×33픽셀이고, PDF 이미지 변환과 clip 기록상 세로는 전체가 표시된다.
오른쪽은 뒤의 이미지와 인접한 clip 때문에 이미지 폭보다 0.12pt 좁다. 이는 이미지
전체가 무조건 표시된다는 증거가 아니라 세로 환산 대조 사례다. 이 문서의 RHWP Native
시각 실행은 별도의 글꼴 환경 검증 오류로 종료돼 그 결과를 시각 통과로 사용하지 않았다.

`curSz` 포함만으로 전체 그림을 판정하는 반례를 한컴에서 확인했다. #7688의 한컴
저장 HWPX에서 두 번째 실제 부분 잘림 그림의 `imgDim`을 0으로 바꾸고 표시 크기와
행렬을 절반으로 변경한 진단 입력을 만들었다. 한컴 2020(11.0.0.9136)에서 HWP로
저장한 뒤 Print method 0, 1-up으로 PDF를 인쇄했다. 저장된 그림의 `curSz`는
23820×2329이고 crop은 `(0,0,47940,4366)`으로 현재 크기를 포함한다. 그럼에도
Print PDF의 두 번째 이미지 객체는 원본 639×70 중 639×61픽셀이다. 원래 기준 PDF의
두 번째 객체는 639×57이다. 따라서 `imgDim` 부재·0 시작·현재 크기 포함만으로 실제
부분 잘림을 전체로 바꿀 수 없다. 합성 입력의 변경 내용과 한컴 저장·인쇄를 구분한다.
진단 HWPX SHA-256 `b276a29c42042c510c2efc401924bad76bc901cf84cb95b05277df62df5c4734`,
한컴 저장 HWP `17e991f8ac1511fb4dc1934982b14aa7149834e218d0e8e7440b2fbb3e5447da`,
Print PDF `ee6d3f2dd596d1336fb38c5d5c82f10834631bb57051c454e24c1fd58e0160b2`다.
입력·인쇄와 MCP 상태는 `output/pr-review/planet6897-ordered-20261031/issue7688-scaled-*`에 있다.
현재 RHWP와 이 합성 대조 PDF의 1쪽 raw는 Native 99.96650%, WASM 98.46927%다.
이는 가설을 반박하는 진단이며 원본 #7688 전체의 새 최종 검증으로 사용하지 않는다.
[Native review](../../output/pr-review/planet6897-ordered-20261031/issue7688-scaled-native-control/issue7688-crop-control/review/review_001.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/issue7688-scaled-native-control/issue7688-crop-control/overlay/overlay_001.png),
[WASM review](../../output/pr-review/planet6897-ordered-20261031/issue7688-scaled-wasm-control/issue7688-crop-control/review/review_001.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/issue7688-scaled-wasm-control/issue7688-crop-control/overlay/overlay_001.png)를 확인했다.

## sample16 35쪽 WASM 정밀 하락

HWP 2022 원본 35쪽의 fresh WASM은 `99.998158639357%`로 #8389 기준 `100%`보다
낮다. Native는 같은 기준과 현재 출력 모두 `100%`다. WASM의 미대응은 독립 Print에만
있는 3픽셀이며, 세 픽셀 모두 본문 글자가 아닌 임베디드 메타파일 그림 내부다. 전체
쪽수나 90% 통과로 하락을 면제하지 않는다. 실제 측정 PNG를 재사용한
[review](../../output/pr-review/planet6897-ordered-20261031/sample16-p035-wasm-review-existing/review/review_035.png)와
[overlay](../../output/pr-review/planet6897-ordered-20261031/sample16-p035-wasm-review-existing/overlay/overlay_035.png)로
해당 영역을 확인했고, 기존 WASM 기준과 메타파일 표시 경로를 대조해야 한다.

### 그림 원본 경계와 자르기 좌표의 추가 대조

#7905 그림은 `imgRect`가 `5810×1610`, 저장 확대 행렬이 `7.6`,
`imgClip`이 `(0, 0, 44200, 12300)`이다. #7688의 축소 반례는 `imgRect`가
`41400×4560`이고 `imgClip`이 `(0, 0, 47940, 4366)`이다. 두 값은 저장
변환과 자르기 좌표계가 함께 관여하므로 개체 크기 또는 `imgRect`와의 단순 포함 관계만으로
독립 `imgDim`을 복원할 수 있다고 판정하지 않는다. 저장소 `samples`와 `tests/fixtures`의
HWPX 954개에서 명시 `imgDim`이 있고 원점이 0인 자르기 중 `imgClip`의 양 끝이
`orgSz` 이상이면서 실제 `imgDim`보다 작은 경우를 검색했으나 일치 사례는 없었다.
이는 해당 임의의 경계 규칙이 안전하다는 증거가 아니다. 검색 명령과 결과는 각각
`output/pr-review/planet6897-ordered-20261031/find-imgrect-crop-counterexamples.py`와
`output/pr-review/planet6897-ordered-20261031/imgrect-crop-counterexamples.tsv`다.

35쪽에서 기존 #8188 CanvasKit 부분 증거 `ck-source-diagram-full.png`와 현재
fresh WASM Canvas2D의 같은 위치를 비교했다. `(421,624)`와 `(421,656)`에서 기존
CanvasKit 그림에는 내용이 있고 현재 Canvas2D의 2px 이웃에는 내용이 없다. 다만
기존 증거는 CanvasKit 부분 렌더이고 현재 전쪽 TSV는 Canvas2D이며 두 Print 래스터의
해당 픽셀 값도 다르므로, 이 비교만으로 동일 backend의 코드 회귀 원인을 확정하지 않는다.
과거 실행의 입력·PDF·프로그램과 각 출력 경로를 따로 대조해야 한다. 픽셀과 비교 명령은
`output/pr-review/planet6897-ordered-20261031/compare-sample16-p035-old-canvaskit-points.py`와
`output/pr-review/planet6897-ordered-20261031/sample16-p035-old-canvaskit-point-comparison.log`에 있다.
기존 CanvasKit 증거의 35쪽 Print 부분과 현재 Print에서 같은 정수 좌표로 자른 부분을
비교하면 RGB가 다른 픽셀은 `96,609/285,792`, 내용 문턱 232에서 불일치는
`11,009`픽셀이다. 기존 증거는 전체 높이 1,123px, 현재 전체 PNG는 1,122px이므로
단순히 같은 정수 좌표를 비교해 출력 회귀의 크기나 위치를 확정할 수 없다. 별도 raster
정책·크기 차이 확인용이며 TSV를 대체하지 않는다. 재현 스크립트·결과는
`output/pr-review/planet6897-ordered-20261031/compare-sample16-p035-print-raster.py`와
`output/pr-review/planet6897-ordered-20261031/sample16-p035-print-raster-comparison.log`다.

전쪽 대조가 진행되며 편람 HWP의 WASM 130·141·145쪽도 기존 정밀 TSV보다
낮은 것으로 확인됐다. 현재 출력과 Print의 원본 PNG를 재사용해 review·overlay를
산출했다. 130쪽은 2px 이웃 내용 미대응이 RHWP 16픽셀·Print 7픽셀이며 모두
삽입된 화면 이미지의 일정표 영역에 있다. 141쪽은 Print 전용 1픽셀 `(496,430)`,
145쪽은 RHWP 전용 12픽셀로 관측됐다. 이 수치는 새 렌더가 아닌 진행 중인 동일
전쪽 측정의 PNG 진단이며 이전 출력의 해당 픽셀과 원인을 증명하지 않는다.
[130쪽 review](../../output/pr-review/planet6897-ordered-20261031/handbook-wasm-drop-review/review/review_130.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/handbook-wasm-drop-review/overlay/overlay_130.png),
[141쪽 review](../../output/pr-review/planet6897-ordered-20261031/handbook-wasm-drop-review/review/review_141.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/handbook-wasm-drop-review/overlay/overlay_141.png),
[145쪽 review](../../output/pr-review/planet6897-ordered-20261031/handbook-wasm-drop-review/review/review_145.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/handbook-wasm-drop-review/overlay/overlay_145.png)와
`output/pr-review/planet6897-ordered-20261031/handbook-wasm-drop-pixel-diagnostic.log`를 참조한다.

130쪽의 과거 PNG를 추가 확인했다. #8394의 정식 `review_130.png`에서 기존 WASM과
Print 패널을 실제 크기 `740×1006`으로 분리하고 현재 전쪽 측정 PNG와 비교한 결과,
각각 RGB 픽셀 전체가 동일하다. 양쪽 2px raw는 미대응 23/합집합 63,928이다.
반면 사용자가 비교 대상으로 지정한 후속 #8397 정밀 TSV는 `99.964042835926%`로,
12자리 표시값을 합리적인 페이지 픽셀 수 범위에서 분수로 복원하면 23/63,965에
대응한다. 따라서 #8394와의 이미지 동일성은 #8397 대비 무하락을 입증하지 않으며,
후속 측정의 실제 PNG·합집합과 글꼴 환경 차이를 확인해야 한다. 정식 기존 패널,
현재 PNG, 정수 통계와 패널 분리 offset 검증은
`output/pr-review/planet6897-ordered-20261031/compare-handbook-p130-historical.py`와
`output/pr-review/planet6897-ordered-20261031/compare-handbook-p130-historical.log`에 있다.

#8397의 `seo-rii-local-fonts.json`에 기록된 한컴바탕·KoPub 글꼴 7개는 현재
시스템의 같은 경로에서 SHA-256이 모두 일치한다. 현재 비교에는 시스템 글꼴 전체가
노출되므로 7개 파일의 바이트 일치만으로 과거와 같은 글꼴 선택 또는 공급 순서를
증명하지 않는다. 확인 로그는
`output/pr-review/planet6897-ordered-20261031/seo-8397-font-hash-check.log`다.
후속 원본 제품 코드 `11def71519`의 WASM을 별도 검증 경로에 복원해 같은 현재 글꼴
환경에서 비교하도록 준비한다. 현재 전쪽 비교는 고정 실행 파일과 별도 `pkg`를 사용한다.

#8040 삽입 원본 4쪽의 #8397 표시값을 현재 미대응 수와 함께 분수로 분석했다.
현재 미대응 수가 이전에도 같았다고 가정할 때 이전 합집합은 각각 215,122·202,557·
180,348·197,514픽셀로 표시된 12자리 값을 재현한다. 현재 합집합은 각각
214,724·202,311·180,227·197,473픽셀이다. 이는 분모 차이만으로도 해당 점수
하락이 생길 수 있음을 보여 주는 조건부 계산이다. 이전의 정수 집계나 픽셀 위치를
확보하지 못했으므로 실제 출력의 동일성이나 원인을 입증하지 않는다. 계산과 한계는
`output/pr-review/planet6897-ordered-20261031/issue8040-score-integer-analysis.py`와
`output/pr-review/planet6897-ordered-20261031/issue8040-score-integer-analysis.log`에 있다.

이미지 자체의 해상도 메타데이터를 확인했다. #3239의 원본 TIFF 네 장은 모두
`1654×2340`, 실제 DPI 태그 `200×200`이다. #7905의 대상 GIF는 `581×161`이며
DPI 메타데이터가 없다. 그러나 실제 자르기 대조군 #7015의 `1181×945` JPEG에는
`300×300 DPI`가 들어 있는데 저장된 오른쪽 자르기 끝 `88,560`은 300 DPI의
24 HWPUNIT/px이 아니라 약 75 HWPUNIT/px과 대응한다. #7688의 JPEG에는 72 DPI가
기록돼 있다. 따라서 이미지의 DPI만 일괄 적용하는 규칙도 근거가 부족하다. 저장 형식과
한컴의 실제 삽입·저장 결과를 추가 대조해야 한다. 입력별 이미지 크기·DPI 로그는
`output/pr-review/planet6897-ordered-20261031/crop-image-resolution-audit.log`다.

원본을 변경하지 않고 #7905의 별도 진단 HWPX에서 해당 그림에만 `imgDim=44200×12300`을
추가해 같은 WASM과 Print PDF로 3쪽을 측정했다. 원본 `99.805284771022%`에서 진단
입력은 raw `100.00%`가 됐다. 두 RHWP PNG에서 바뀐 9,755개 RGB 픽셀의 외접 범위는
`(94,124)~(676,286)`으로 해당 그림 내부다. 원본에 없는 전체 크기를 실험적으로 부여한
것이므로 원본 출력이 100%라는 증거나 제품 보정이 아니다. 자르기 기준이 차이를 만드는지
확인한 결과이며 실제 부분 잘림·DPI 반례를 보존하는 규칙이 여전히 필요하다.
[진단 review](../../output/pr-review/planet6897-ordered-20261031/issue7905-imgdim-diagnostic-wasm/issue7905-imgdim-diagnostic/review/review_003.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/issue7905-imgdim-diagnostic-wasm/issue7905-imgdim-diagnostic/overlay/overlay_003.png),
[TSV](../../output/pr-review/planet6897-ordered-20261031/issue7905-imgdim-diagnostic-wasm/issue7905-imgdim-diagnostic/silhouette.tsv),
`output/pr-review/planet6897-ordered-20261031/issue7905-imgdim-diagnostic-delta.log`를 참조한다.

#7015의 한컴 Print PDF 3쪽에는 해당 JPEG가 `1181×329`로 포함돼 있다.
원본 잘림 높이 `45,453−20,745=24,708` HWPUNIT을 75로 나누면 약 329.44픽셀이어서,
JPEG에 기록된 300 DPI를 그대로 사용하는 가정보다 이 Print와 부합한다. 다만 이 사례
하나로 다른 형식이나 저장 방식에 같은 기준을 일반화하지 않는다.

#7688에서 한컴 저장·Print로 부분 잘림을 확인한 진단 입력에만 별도로 전체 기준을
`639×75`·`70×75`로 부여한 후 같은 Print와 비교했다. 기준이 없는 진단 입력 대비
Native raw는 `99.966501656307% → 99.966492926284%`, WASM은
`98.469273743017% → 98.468874566926%`다. 두 경로의 미대응 수는 각각 9·411로
같고 합집합이 각각 7픽셀 줄었다. 한쪽 점수도 개선되지 않았고 두 입력 모두 진단용이므로
75 HWPUNIT/픽셀을 새 공통 규칙으로 채택하지 않았다. 입력 생성·측정·정수 비교는
`issue7688-reference75-diagnostic-create.py`, `issue7688-reference75-diagnostic-native.log`,
`issue7688-reference75-diagnostic-wasm.log`, `issue7688-reference75-compare.log`에 있다.

#7905 그림의 실제 부분 잘림 확인을 위해 원본 HWPX의 해당 GIF에서만 자르기 끝을
`(22000,6000)`으로 바꾼 별도 진단 입력을 만들었다. 한컴 2020 MCP로 HWP 저장 후
해당 HWP를 Print했고, 다시 HWPX로 저장해 바뀐 자르기와 `imgDim` 부재가 보존됨을
확인했다. Print PDF 3쪽의 `/x59`는 원본 581×161 그림이며, PDF 이미지 변환과 clip은
각각 가로 874.000031pt 대 441pt, 세로 245.999996pt 대 122pt다. 저장 좌표를
픽셀에 역산하면 약 75.04·75.15 HWPUNIT/픽셀이다. PDF 좌표의 인쇄 반올림을 포함하므로
이 수치만으로 정확한 단위 정책을 단정하지 않는다. #7688에서도 PDF 이미지 자체의
61행뿐 아니라 clip을 함께 적용하면 약 57.73행이 보이며, 저장 자르기 높이의
75 HWPUNIT/픽셀 환산은 약 58.21행이다. #7015 Print의 독립 JPEG 높이는 329픽셀이고
75 환산은 약 329.44픽셀이다.

현재 후보가 한컴 저장 진단 HWP를 출력하면 같은 Print와 비교한 3쪽 raw는 Native
`89.24512%`, WASM `89.19755%`이며 RHWP에는 전체 그림이 보이지만 Print에는 부분만
보인다. 이는 실제 한컴 Print로 확인한 새로운 반례이며, 수동 수정 문서에 관한 진단이지
원본 문서의 전쪽 점수나 완료된 회귀 검사가 아니다. [review](../../output/pr-review/planet6897-ordered-20261031/issue7905-gif-partial-control-wasm/gif-partial/review/review_003.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/issue7905-gif-partial-control-wasm/gif-partial/overlay/overlay_003.png),
`crop-coordinate-print-geometry.log`, 한컴 MCP 작업의 `issue7905-gif-partial-*` 상태·
출력 및 `issue7905-gif-partial-control-{native,wasm}.log`를 보존한다. 한컴이 다시 저장한
HWPX에서 전체 크기를 75 기준으로 명시한 진단과 명시하지 않은 진단을 비교 중이다.

같은 부분 잘림 진단 HWP를 한컴이 다시 저장한 HWPX에서도 원본과 같은 Print에 대한
WASM raw는 `89.19755%`다. 이 HWPX의 해당 그림에만 `imgDim=43575×12075`
(581×161픽셀의 75 HWPUNIT/픽셀)을 추가한 진단은 `100.00%`다. 반대로 #7905의
자르기 경계가 전체 이미지보다 큰 원본 사례에서 같은 75 기준을 단순 명시하면
`96.14%`가 됐다. 따라서 부분 자르기의 기준 산출과 이미지 범위를 초과하는 선택의
처리는 별개이며, 75 값 하나만 넣는 보정으로는 두 계약을 동시에 만족하지 않는다.
결과는 `issue7905-gif-partial-roundtrip-wasm.log`,
`issue7905-gif-partial-reference75-wasm.log`, `issue7905-reference75-diagnostic-wasm.log`에 있다.

독립적인 GIF 원본을 찾기 위해 저장소 HWPX를 조사했고, 별도 Print가 있는
`tests/fixtures/issue7661_backgrounds/verification/api-issue7235.hwpx`의 첫 쪽에서
`134×91` GIF·`imgDim=0`·전체에 가까운 crop을 확인했다. 해당 원본과 Print를 현재
WASM으로 확인하고 이미지 전체 기준을 명시한 진단본도 같은 PDF와 대조 중이다.

#8397 당시 소스의 표준 최적화 WASM 빌드는 컴파일 중 종료 코드 143으로 중단됐다.
로그만으로 원인은 확인하지 못했다. 원래 후보와 같은 글꼴 환경의 출력 차이를 먼저
관측하기 위해 같은 소스·공용 target·8개 빌드 작업으로 `--no-opt` 진단 패키지를
별도 경로에 빌드한다. 이 진단 패키지는 정식 fresh optimized WASM 검증을 대체하지
않으며, 정식 비교에는 표준 최적화와 실제 패키지 해시 확인이 별도로 필요하다.

독립 GIF 대조군 `api-issue7235.hwpx`의 1쪽을 기존 Print와 비교했다. 원본은
`99.706327541819%`이고 해당 그림에만 전체 기준을 명시한 진단 입력은
`99.706384519868%`다. 미대응 픽셀은 모두 454개이고 합집합만 154,594에서
154,624로 달라졌다. RHWP PNG의 RGB 차이 4,260개는 `(82,132)~(202,218)`의
그림 영역에 한정된다. 이는 별도 GIF에서도 기준 선택이 실제 그림 표시를 바꾼다는
근거이지만 미대응 수가 같으므로 새 규칙의 일반적 정확성을 증명하지 않는다.
비교 스크립트·정수 결과는 `issue7235-gif-original-vs-full.py`와
`issue7235-gif-original-vs-full.log`다.

같은 GIF에서 자르기 사각형 전체가 이미지 밖까지 나가도록
`(-5000,-2000,50000,16000)`을 준 별도 진단 HWP를 한컴 2020 MCP로 저장·Print했다.
다시 저장한 HWPX에도 음수 시작·초과 끝점과 `imgDim` 부재가 보존된다. Print PDF
3쪽에서 그림은 전체 영역으로 표시되고 `/x59`의 변환 폭·높이와 Print clip은 각각
약 `442×122pt`·`441×122pt`다. 따라서 이 진단에서는 범위 밖 여백을 그대로 확대해
나타내지 않는다. 이어 왼쪽만 음수이고 나머지는 부분 선택인
`(-5000,0,22000,6000)` 진단을 저장·Print한 결과, PDF의 그림 transform·clip은
왼쪽 시작이 0인 같은 부분 선택 진단과 동일했다. 저장본의 속성 보존 여부와 다른
경계·다른 원본을 더 확인한 뒤 공통 범위 계산에 반영해야 한다. 입력·저장·Print·
페이지별 PDF 연산자는 `output/pr-review/planet6897-ordered-20261031/issue7905-gif-overflow-*`,
`issue7905-gif-mixed-*`에 보존한다.

구현 전 추적: 원본 `Picture.crop/imgDim`은 layout에서 `ImageNode.crop/original_size_hu`로
전달되고 `RenderNode::new`에서 실제 그림 노드가 만들어진다. 이후 `PaintOp::image`,
SVG·Native Skia·Canvas는 그 노드의 같은 crop과 기준 크기를 소비해 source rectangle을
계산한다. GIF가 명시 기준 크기를 보존한 경우는 그 값을 그대로 사용하고, 기준이 없는
GIF만 픽셀 크기와 한컴 Print에서 확인한 96 DPI 단위를 이용해 이미지의 유효 영역과
교차시킨다. 저장된 모델의 crop은 변경하지 않는다. 직접 생성된 PaintOp도 같은 공통
정규화를 소비하게 하며, 무효·영역 밖 선택을 전체 그림으로 바꾸지 않는다. 부분 잘림,
전체를 덮는 범위, 음수 시작·부분 끝, 명시 `imgDim`, 비-GIF TIFF/JPEG와 #8410의
작은 패턴 선택을 대조한다. 진단 입력의 Print는 합성 속성에 대한 실제 한컴 실행이며,
독립 원본 자체로 간주하지 않는다.

음수 시작의 부분 선택을 한컴 저장본에서 HWPX로 다시 읽어
`imgClip=(-5000,0,22000,6000)`, `imgDim` 부재가 그대로임을 확인했다.
그 Print PDF 3쪽과 시작점이 0인 부분 선택 Print PDF 3쪽의 96 DPI PNG SHA-256은
둘 다 `3c0e209cf8cd0092e6cbdc5159bd862bf44e1ef376016304335132f9bdbc8cdd`다.
이는 이 두 진단에서 음수 시작 영역이 보이는 공백으로 늘어나지 않는 독립 출력 증거다.

제품 후보에는 `image_geometry::normalize_gif_crop` 공통 결과를 그림 노드 생성과 직접
PaintOp 생성에서 소비하도록 추가했다. 명시 기준은 보존하고, GIF 헤더와 유효 픽셀
크기가 있는 경우에만 75 HWPUNIT/픽셀 범위와 자르기를 교차한다. 기존 모델 원본을
수정하지 않으며 명시 기준·TIFF·JPEG는 그대로다. 현재 후보 빌드·실제 원본 6쪽 및
부분·범위 초과 진단과 정상 대조군의 Native/fresh WASM 재검증 전이므로 완료 판정이
아니다. 기존 표준 WASM 전쪽 비교는 수정 전 고정 실행 파일을 사용해 계속 진행한다.
독립 GIF 대조군의 같은 1쪽 Native raw는 원본 `99.948561680405%`, 전체 기준을
명시한 진단본 `99.948576412847%`다. 각각 미대응 79픽셀이며 합집합은
153,582·153,626픽셀이다. 이 자료도 현재 제품 보정 후 측정은 아니다.

#8397 당시 소스의 비최적화 WASM 패키지는 컴파일·패키징 메시지 뒤 세션이 종료 코드
143으로 끝났다. 바이너리 유효성 검사와 실제 3쪽 실행은 성공했으나 정식 최적화 빌드
완료 증거로 사용하지 않는다. 같은 현재 시스템 글꼴 조건에서 #7905 3쪽은 과거 진단
`99.805340653856%`(미대응 156/합집합 80,140), 현재 `99.805284771022%`
(156/80,117)이다. 두 RHWP PNG의 RGB 차이 201픽셀은 `(67,1045)~(84,1059)`의
하단 정부 로고에만 있다. 독립 Print와 과거·현재 확대 판독에서 현재 로고만 축소 시
계단형이고 과거 출력은 Print처럼 부드럽다. 156개 미대응의 큰 그림 오른쪽 경계는
과거와 현재 모두 빠진다. 기록된 #8397 정밀 TSV와 진단의 글꼴·빌드가 같다고 추정하지 않는다.

코드 추적에서 #8410이 1픽셀 미만의 source 영역 차이도 실제 자르기로 보존한 이후,
Canvas2D의 전체 그림 경로는 축소 시 `ReductionQuality::for_full_image`를 적용하지만
실제 crop의 9인자 `drawImage` 경로는 같은 축소 품질을 적용하지 않는다. 따라서 자르기
영역 자체를 전체로 바꾸지 않고 해당 영역의 크기로 축소 품질을 결정하는 공통 처리의
필요성을 후보로 조사한다. 이는 아직 실제 보정·검증 전의 원인 가설이다. PNG 차이 및
확대는 `issue7905-old-wasm-current-delta.log`, `issue7905-old-current-print-footer-focus.png`다.

sample16 HWP 2022 35쪽을 #8397 소스 `11def71519`의 비최적화 진단 WASM과 현재
최적화 WASM으로 같은 현재 글꼴·입력·Print 조건에서 실행했다. 두 RHWP PNG SHA-256은
모두 `9e862f1d33dc8bca6360b17bd3e70613dbf322928356df603875ce3fef611902`이며,
raw 정수 집계도 모두 미대응 3/합집합 162,923, `99.998158639357%`다. 이 조건에서는
기존 소스와 현재 출력이 바이트 단위로 같다는 직접 증거다. 다만 과거 패키지 빌드
세션은 143으로 종료됐고 비최적화였으며, 과거 정식 TSV의 100%와 측정 환경 차이가
해소된 것은 아니다. 원래 사용자 기준을 충족했다고 판정하지 않는다. 로그는
`baseline-8397-unopt-sample16-wasm.log`, 현재 결과는
`crop-fix-frozen-all/preserve-system-wasm/sample16-hwp2022/`에 있다.

#8397 소스 `11def71519`의 비최적화 진단 WASM과 현재 최적화 WASM을 현재 설치 글꼴
전체로 같은 편람 HWP·Print에 적용해 43·130·141·145쪽을 비교했다. 각 쪽의 RHWP
PNG SHA-256이 이전 코드와 현재 코드에서 각각 완전히 같다. raw 미대응/합집합도 차례로
`4/100375`, `23/63928`, `1/112259`, `12/150274`로 같다. 현재 글꼴·도구 환경에서
이 네 쪽은 코드 출력 보존이 직접 확인되나, 당시 공개 TSV와 합집합 등이 다른 원인과
사용자가 요구한 공개 정밀 수치 조건은 남는다. 이전 빌드 세션은 143으로 종료됐고
패키지는 비최적화이므로 이 비교는 정식 최종 WASM 검증을 대체하지 않는다. 로그와
전쪽 측정 PNG는 `baseline-8397-unopt-handbook-wasm.log` 및
`baseline-8397-unopt-handbook-wasm/handbook-old/`에 있다.

GIF 공통 보정을 포함한 Native release-test 빌드는 종료 코드 0이다. #7905 원본 전체
6쪽의 raw는 `99.98/98.86/100/100/100/100%`이며 정밀 수치도 #8397에 기록된
#8389 대조 6쪽 모두보다 높다. 한컴 저장·Print한 부분 선택과 음수 시작 부분 선택도
각 3쪽 raw 100%다. 별도 GIF #7235, JPEG #7015, 200 DPI TIFF #3239의 선택 쪽도
90% 이상이다. #7015 3쪽과 #3239 1쪽은 수정 전후 Native PNG SHA-256이 각각 같아
비-GIF 대조군의 출력 변경이 없음을 확인했다. 실행 파일 SHA-256과 각 명령·종료 결과는
`rhwp-gif-crop-candidate.sha256`, `gif-crop-candidate-native-status.log`,
`gif-crop-control-old-vs-new-native.log` 및 각 `gif-crop-candidate-native-*.log`다.
[원본 3쪽 review](../../output/pr-review/planet6897-ordered-20261031/gif-crop-candidate-native/issue7905/issue7905/review/review_003.png)·
[overlay](../../output/pr-review/planet6897-ordered-20261031/gif-crop-candidate-native/issue7905/issue7905/overlay/overlay_003.png)를 확인했다.

Canvas2D에서는 잘린 source 자체를 유지한 채 축소 비율에 따라 전체 그림과 같은
`ReductionQuality::for_source_window`를 선택하도록 공통 축소 품질 helper를 사용했다.
이 변경은 `wasm32` 전용 모듈이며 기존 Native 빌드 결과를 변경하지 않는다. GIF 보정과
함께 소스 해시를 고정해 표준 `wasm-pack --release` 최적화 빌드를 별도 패키지에 시작했고,
아직 WASM 실행·#8410 작은 타일과 전쪽 결과는 확인하지 않았다. 빌드 로그는
`gif-crop-candidate-wasm-build.log`다.

첫 최적화 WASM 빌드는 종료 코드 1이었다. 새 Canvas 품질 guard가 `self.ctx`를 빌린 채
이후 `&mut self` 경로를 호출해 Rust `E0502`가 발생했고, guard가 소유할 Canvas 핸들
복제본을 별도로 만들어 대여 수명을 분리했다. 또 과거 작업 디렉터리의 같은 이름 PDF
의존성이 공용 target에 재사용돼 현재 소스에 존재하는 타입·오류 변형을 찾지 못했다.
공유 target을 삭제하지 않고 현재 `vendor/rhwp-pdf-import-backend/src/lib.rs`의 시각을
갱신해 Cargo가 현재 경로의 의존성을 재컴파일하도록 했다. vendor 바이트는 변경하지
않았다. 표준 최적화 WASM 재빌드가 실제 `/home/tsjang/rhwp/vendor/...`를 컴파일하는
것을 확인했으며 결과는 `gif-crop-candidate-wasm-retry.log`에서 확인한다. 최초 실패는
`gif-crop-candidate-wasm-build.log`에 보존한다.

브랜치 기준 `tests/cases/` 변경 파일 33개를 첫 줄 검사 유형·주석 갱신 시각으로
다시 점검했다. 기존 점검표는 이전 변경을 반영하지 않아 현재 상태로 갱신했다.
실제 assertion이 PDF 가져오기 IR·저장 축 관계를 검사하고 페이지 래스터 점수를
판정하지 않는 다섯 파일(`issue_8303`, `issue_8355`, `issue_8362` 두 파일,
`issue_8381`)은 유형·실제 갱신 시각·직전 파일 커밋·입력과 검사 범위를 첫머리에
추가했다. 과거 시각 수치를 새로 추정하지 않았으며 assertion은 바꾸지 않았다.
이 다섯 파일의 rustfmt 검사와 `git diff --check`가 통과했다. 주석만 바꾼 이유로
전체 회귀는 다시 실행하지 않았다. 나머지 13파일은 실제 검사와 독립 시각 근거를
확인해 같은 방식으로 보완한다. 현재 목록은
`output/pr-review/planet6897-ordered-20261031/current-regression-comment-audit.tsv`다.

#8040의 `㉠`에 대한 현재 Native 글꼴 결정 추적을 추가 확인했다. 원본의 기호 슬롯은
`한양신명조`이며, 해석된 요청 이름은 `HY Sinmyeongjo`다. 실제 glyph 공급자는 시스템의
`Haansoft Batang`으로 기록됐다. 별도로 추출한 독립 Print의 해당 원문자 글꼴은 Gulim과
대응한다. 이는 현재 대체 글꼴 경로를 조사할 근거이지만, 이 관측만으로 네 쪽의 raw 하락
전체 원인이나 Gulim으로 일괄 변경해야 한다고 단정하지 않는다. 앞서 본문 전체에 Gulim을
강제한 실험은 일부 쪽을 악화시켰으므로 그대로 적용하지 않는다. 문자별 추적은
`output/pr-review/planet6897-ordered-20261031/issue8040-native-font-trace-final.json`,
Print 글꼴 대조는 `issue8040-print-symbol-font-matches.json`에 있다.

GIF·Canvas 보정 전 고정 WASM 산출물의 전체 대조가 종료됐다. 공개 기준 TSV 2,153행 중
2,145행은 유지·개선, 8행은 하락이며 누락은 없다. #7905 3쪽과 편람·sample16 쪽이
포함된다. 현재 제품 코드에 추가된 GIF·Canvas 보정의 결과는 아니며, 이 수치로 최종
통과를 선언하지 않는다. `crop-fix-frozen-comparison-wasm-final.log`와
`crop-fix-frozen-all/comparison-replay-bc9-wasm-original.tsv`에 전 행을 보존한다.

#8040 추가 진단으로 원본 얼굴은 유지하고 `HY Sinmyeongjo`의 glyph 폴백 후보에만
Gulim을 지정해 Native 전 4쪽을 재출력했다. raw는 순서대로
`99.958965941732 / 99.999505865378 / 99.813608778139 / 99.717904695342%`다.
3쪽은 이전 후보보다 나아졌지만 #8397 기준과 비교하면 1·2·4쪽이 여전히 하락한다.
이 글꼴 후보 변경만으로 사용자 기준을 충족하지 못하므로 제품 기본값에 적용하지 않는다.
실행 로그는 `issue8040-fallback-only-native.log`, 설정은
`issue8040-fallback-only-diagnostic.json`이다.

개별 원 PR 리뷰 23건의 오래된 전쪽 대조 진행 문구를 실제 완료된 고정 후보 대조 결과
(각 2,153행 중 Native 12행·WASM 8행 하락)로 갱신했다. 이 결과는 뒤이어 적용한
GIF·Canvas 보정의 재검증 결과가 아니며, 각 리뷰의 보류 판정은 유지한다.

표준 최적화 WASM 빌드 종료를 기다리는 별도 검증 세션을 준비했다. 빌드 종료 코드 0,
빌드 전후 및 검증 직전 소스 해시 일치, WASM 바이너리 컴파일 확인을 모두 통과한 뒤에만
#7905 6쪽과 부분 GIF·음수 시작 GIF·JPEG·TIFF·다른 GIF 대조군을 실행한다. 시스템
글꼴 자동 공급을 사용하며 실행 경과는 `gif-crop-candidate-wasm-status.log`에 기록한다.
이 준비는 실행 성공이나 정밀 기준 통과를 뜻하지 않는다.

최적화 WASM 빌드는 종료 코드 0이며 빌드 전후 소스 해시가 일치했다. 패키지의
`rhwp_bg.wasm` SHA-256은 `e328e845af511449b15093e01282a3413a4351c3a6ae6b67eeea33af3a713706`다.
바이너리 컴파일 확인과 검증 직전 소스 해시 확인을 거쳐 #7905 6쪽 WASM 대조가 실제로
시작됐다. 로그는 `gif-crop-candidate-wasm-retry.log`와
`gif-crop-candidate-wasm-issue7905.log`다. 전쪽 결과와 기존 TSV 대비 정밀 판정은 아직 없다.

위 #7905 전 6쪽 WASM 대조는 종료 코드 0이며 각 raw는
`99.980697560469 / 98.866573169979 / 100 / 100 / 100 / 100%`다. 기존 #8389 대조에
기록된 #8397 후보 값과 정밀 대조한 6쪽 모두 유지·개선이고 전쪽 98% 이상이다.
정수 미대응·합집합으로 재계산한 결과는 `gif-crop-issue7905-wasm-compare.log`다.
실제 산출 PNG로 만든 3쪽 review·overlay는
`gif-crop-candidate-wasm/issue7905/issue7905/review/review_003.png`와
`gif-crop-candidate-wasm/issue7905/issue7905/overlay/overlay_003.png`다. 부분 자르기와
다른 형식 대조군 및 빈 선택의 WASM 결과는 별도로 확인해야 한다.

한컴에서 저장·Print한 부분 GIF 자르기 진단 3쪽은 Native와 새 최적화 WASM이 각각
raw 100%다. 자르기 전 전체 그림으로 확장하지 않고 원본의 일부만 보존하는지 직접
판독했다. WASM의 같은 출력으로 만든 자료는
`gif-crop-candidate-wasm/gif-partial/gif-partial/review/review_003.png`와
`gif-crop-candidate-wasm/gif-partial/gif-partial/overlay/overlay_003.png`다.

새 최적화 WASM의 나머지 선택 대조도 종료 코드 0이다. 음수 시작 GIF 3쪽은 raw
100%, JPEG #7015 3쪽은 99.96%, 200 DPI TIFF #3239 1쪽은 100%, 별도 GIF #7235
1쪽은 99.71%다. 이는 각 선택 쪽의 한컴 Print 대조이며 이 수치만으로 해당 문서의
전쪽 검증이나 기존 모든 TSV 유지가 입증되는 것은 아니다. 각 결과는
`gif-crop-candidate-wasm/<대상>/<대상>/silhouette.tsv`와 같은 이름의 실행 로그에 있다.

#7015 3쪽의 수정 전 고정 WASM과 새 최적화 WASM을 같은 시스템 글꼴 및 Print로
별도 실행했다. RGB 변경 픽셀의 외접 범위 `(221,865)-(570,949)`는 렌더 트리의
그림 틀 `(200.8,852.6,392.1,107.5)` 안이다. raw 미대응은 양쪽 15개이며 합집합은
41,705→41,812로, 정밀 raw는 `99.964033089558%`→`99.964125131541%`다. 같은 쪽의
출력 유지·개선을 확인했지만 다른 문서의 무회귀 증거로 일반화하지 않는다. 해시와 좌표는
`gif-wasm-issue7015-before-after.log`에 있다. #3239 동일 조건 대조는 진행 중이다.

#3239 1쪽도 수정 전 고정 WASM과 같은 조건으로 대조했다. 새 WASM의 PNG SHA-256은
이전 출력과 같고 RGB 변경 픽셀은 0개이며, Print 대비 정밀 raw는 양쪽 100%다.
해시와 정수 집계는 `gif-wasm-issue3239-before-after.log`에 있다.

#7905 원본 전쪽 Native·fresh WASM이 90% 이상이고 3쪽을 직접 판독한 뒤, 해당 GIF가
불필요한 중첩 자르기 창에 들어가지 않는 글꼴 독립 관계 검사를
`tests/cases/integration_gif_full_source_ownership.rs`에 추가했다. 수정 전 고정
실행 파일의 SVG에서는 조건이 실패하고 현재 후보 SVG에서는 성립함을 직접 확인했다.
정식 증거는 `mydocs/pr/assets/integration_20261031_gif_crop/`에 보존한다. 최소 글꼴
환경의 새 검사 실행은 `gif-source-regression.log`에서 진행 중이며 종료 결과를 확인해야 한다.
수정 전·후 고정 실행 파일의 SVG를 같은 관계 조건으로 검사한 별도 진단은 각각 FAIL·PASS다.
이 진단은 Rust 회귀 실행을 대신하지 않으며 `gif-source-before-after-assertion.log`에 남겼다.
새 Rust 검사는 최소 글꼴 환경에서 1건 실행·1건 통과하고 종료 코드 0이었다. 실제 노출
글꼴 5개와 nextest 결과는 `gif-source-regression.log`에 있다. 다른 304건은 선택
필터로 제외됐으며 이 실행을 전체 회귀 통과로 기록하지 않는다.
새 그림 처리 변경 후 #8410의 기존 Native Skia 관련 6건도 다시 확인하도록 현재 GIF
집중 회귀가 끝난 뒤 실행하는 대기 세션을 만들었다. 새 manifest가 배정한 suite 018·026을
사용하며, Cargo 검증은 같은 `target/pr-review`에서 동시에 실행하지 않는다. 실제 실행·종료
결과는 `gif-final-8410-native-skia-tests.log`를 확인한다.
실행 결과는 최소 글꼴 환경에서 6건 실행·6건 통과, 종료 코드 0이다. 필터로 제외된
589건은 이 실행의 통과 건수에 포함하지 않는다. GIF 관련 수정 파일과 새 회귀의 rustfmt
검사도 통과했다(`gif-format-check.log`).

#8410의 공개 패턴 HWP/HWPX 저장본을 새 Native·WASM으로 독립 원본 PDF와 전쪽
대조한다. 현재 Native 2쪽 raw는 99.87%로 과거 기록 99.98%보다 낮다. 과거 공개 패턴
기록은 MuPDF, 이번 sweep은 PDFium을 사용하므로 이 두 수치만으로 코드 회귀를 확정하지
않는다. 같은 PDFium·시스템 글꼴 조건의 이전 고정 실행 파일 대조도 순차 실행하도록 했다.
새 HWP 3쪽의 실제 반쪽 타일 4개는 SVG에서 각 타일의 틀을 읽고 그 내부를 상대 위치로
검사했으며 Native·WASM 모두 각 빨강·파랑 원본 열을 보존했다. 실행 경과는
`gif-final-8410-visual-status.log`, 색상 검사는
`gif-final-8410-pattern-hwp-color-check.log`, 이전 후보는
`gif-before-8410-visual-status.log`다. 전체 경로의 대조가 끝나기 전까지 기능 보존으로
최종 판정하지 않는다.

패턴 HWP 2쪽의 동일한 현재 Native·WASM PNG를 두 독립 PDF 렌더와 각각 대조했다.
PDFium은 양쪽 raw `99.87421%`(미대응 9/합집합 7,155), MuPDF는 양쪽
`99.98481%`(1/6,584)다. 기존 #8410 문서의 MuPDF 99.98% 표시와 현재 MuPDF의
소수 둘째 자리 수치가 일치하며, 서로 다른 기준 렌더러가 이번 2쪽 수치 차이를 만든다는
직접 근거다. 이는 같은 조건의 이전·현재 출력 비교를 대체하지 않는다. 재현 결과는
`issue8403-pattern-p002-reference-renderers.log`에 있다.

같은 시스템 글꼴과 PDFium 조건에서 GIF 보정 전 고정 Native 및 현재 Native의 공개 패턴
HWP·HWPX를 각각 전 5쪽 실행했다. 두 입력 모두 5쪽 전부 RHWP PNG SHA-256이 같다.
즉 Native에서 해당 패턴 출력의 픽셀 변화는 없으며, 같은 조건의 WASM 전쪽 비교는
진행 중이다. Native 해시 비교는 `issue8403-pattern-native-same-env-compare.log`다.
HWPX 저장본 3쪽에서도 별도로 SVG의 반쪽 타일 4개를 찾아 상대 위치의 색상을 검사했다.
Native·WASM 모두 빨강/파랑 열을 보존했다(`gif-final-8410-pattern-hwpx-color-check.log`).
WASM 직접 판독 자료는
`gif-final-8410-visual/patterns-hwpx-wasm/patterns-hwpx-wasm/review/review_003.png`와
같은 경로의 `overlay/overlay_003.png`다.
HWP 저장본의 변경 전·후 WASM PNG도 같은 시스템 글꼴·PDFium 조건에서 전 5쪽 모두
SHA-256이 같다(`issue8403-pattern-hwp-wasm-same-env-compare.log`). HWPX WASM 대조는
별도로 계속한다.
HWPX 저장본의 변경 전·후 WASM도 종료 코드 0이며 전 5쪽 PNG SHA-256이 모두 같다.
따라서 이 공개 패턴의 HWP/HWPX × Native/WASM 네 경로에서 GIF 보정 전후 출력이
동일함을 확인했다. HWPX WASM 비교는
`issue8403-pattern-hwpx-wasm-same-env-compare.log`다. 이 결과는 #8410의 다른 문자 획·
비공개 원본 전체까지 자동으로 검증한 뜻은 아니다.

2026-11-01 KST에 GitHub API로 원 PR head를 재확인했다. #8161은
`bd786fc5fefb94126ac146411f603c9cbcf336a6`, #8346은
`923ae70036a6b6e5ce94bf9c45cd2f8326cf8551`로 개별 리뷰에 기록한 값과 같으며,
이번 확인 시각까지 두 원 PR head 변경에 따른 재체리픽 사유는 없다.

같은 날 원격 `upstream/devel`을 `git ls-remote`로 확인한 결과
`bc9dfdd057258de59afebeb5121c4bee7398e07f`이며 로컬 추적 ref와 현재 브랜치의
merge-base가 모두 같다. 제출 직전에는 원격 head를 다시 확인한다.

#8040 원문자를 둘러싼 한글 슬롯은 `신명 중명조`, 해당 기호 슬롯은 `한양신명조`임을
원본 글꼴 결정 추적과 HWPX 내보내기의 face 목록에서 확인했다. 기호 슬롯의 해당 레코드만
독립 Print에서 확인한 Gulim으로 바꾸고 나머지 슬롯은 보존한 Native 진단은 4쪽 순서로
`99.958956372490 / 99.999505977670 / 99.813762713743 / 99.726738625495%`다.
3·4쪽은 #8397 기준보다 높지만 1·2쪽은 낮으므로 이 실험을 제품 기본값으로 반영하지
않았다. 첫 실행은 전역 치환과 문서별 치환이 같은 원본 이름을 중복해 환경 유효성 검사에서
실패했고, 두 번째는 각 슬롯별 원본 레코드 주소로 분리해 정상 종료했다. 설정과 로그는
`issue8040-symbol-slot-only-diagnostic.json`, `issue8040-symbol-slot-only-native-v2.log`다.

1쪽의 대표 `㉠`과 인접 글자를 같은 영역에서 확대해 이전 base·현재 후보·기호 슬롯
대체 진단·Print를 직접 비교했다. 해당 영역의 2px raw 미대응은 세 출력 모두 0이지만
합집합은 각각 141·141·134픽셀이었다. 따라서 이 영역에서는 100%라는 보조값만으로
글리프 모양이 같다고 판단할 수 없다. 전쪽 1·2쪽의 미대응 개수도 이전 base와 후보가
같지만 합집합이 달라 정밀 백분율은 하락한다. 확대 자료는
`issue8040-p001-slot-diagnostic-detail.png`이며, 현재 사용자 기준의 하락을 면제하는
근거로 사용하지 않는다.

#8040 1~4쪽에서 이전 고정 Native와 현재 Native의 2px 미대응 마스크를 직접 대조했다.
양쪽 미대응은 각각 88·1·337·557픽셀이며 네 쪽 모두 미대응 위치 자체도 같다.
따라서 정밀 백분율 변화는 해당 마스크의 차이가 아닌 내용 합집합 크기 변화다. 1쪽의
미대응은 `(165,376)-(418,389)` 범위의 `뜨거워진 한반도, 과일 재배 지도가 바뀐다!`
본문에 있고, Print는 이 제목을 내장 Type3 `T5`로 출력한다. 이전 base의 `㉠`은 문맥상
한글 슬롯의 `Sinmyeong Jungmyeongjo` 또는 `Sinmyeong Junggothic`을 사용했으나 현재는
#8136의 기호 슬롯 `HY Sinmyeongjo`를 사용한다. 이 관찰은 #8136을 되돌리거나 기존
정밀 TSV 하락을 면제하는 근거가 아니다. 정수 마스크 대조는
`issue8040-before-current-unmatched-mask.log`에 있다.

같은 1쪽 제목의 독립 Print에서 `뜨거워진한반도`는 내장 Type3 `T5`이고 첫 글자의
96dpi 환산 bbox 시작은 x=164.80이다. 현재 SVG의 대응 글자 원점은 x=164.0667이며
원본 글꼴은 HFT `신명 중고딕`, Native 글꼴 추적의 표시 family는
`Sinmyeong Junggothic`이다. 독립 Print의 glyph bbox와 SVG 원점은 서로 다른 값이므로
이 차이만으로 배치 오차를 확정하거나 고정 보정값을 적용하지 않는다. 시스템에는 같은
family의 언어별 OTF가 여러 개 있고 한글 `뜨`는 `TEJGTHG.otf`의 cmap에서 확인된다.
실제 선택된 개별 프로그램과 Type3 윤곽·조판 기준점의 대응을 추가로 확인해야 한다.
독립 Print의 Type3 `T5` ToUnicode에서 `뜨`는 코드 `03`, `/CharProcs`의 `/HFT37`에
대응한다. 이 glyph의 외곽과 세 윤곽 제어점 범위는 시스템
`TEJGTHG.otf`의 `hft9B61`과 각각 `(54,33)-(946,102)`, `(150,312)-(467,690)`,
`(518,323)-(845,689)`로 같다. 두 glyph의 전체 경로와 실제 Native가 선택한 파일이
같다는 증명은 아니며, glyph 외곽 모양의 유력한 대응 근거로만 사용한다.
두 윤곽을 같은 1,000×1,000 격자에 각각 래스터화하면 채우기만 비교한 RGB 차이는
0픽셀이다. Print의 Type3 명령은 이 윤곽에 `20/1000 em` 획과 채우기를 함께 적용한다.
양쪽에 같은 획을 추가한 진단에서는 RGB 차이가 7픽셀이다. 이는 글자 단위의 대응
근거이며, Native에서 실제 선택된 글꼴 파일과 장치 해상도·획 적용 결과를 확인하기 전에는
전체 제목의 원인을 확정하지 않는다. 진단 이미지와 중간 SVG는
`issue8040-glyph-{pdf,otf}[-stroke]-diagnostic.*`에 있다.
같은 제목의 `뜨거워진한반도` 일곱 글자에 대해 PDF text origin과 현재 SVG 원점을
96dpi 공통 좌표로 비교하면 PDF−RHWP x는 약 +0.6266~+0.7337 CSSpx, y는 공통
−0.2236 CSSpx다. x 차이는 세 값에 가깝게 반복된다. 이 결과는 한 문단에 대한
관측이며 PDF 좌표 양자화와 표·문단 원점 중 원인 계층을 확정하지 않는다. 특정 좌표를
더하는 보정 없이 저장 원본·공통 배치 경로를 계속 대조한다. 결과는
`issue8040-title-origins.log`다.
제목의 쉼표 전후로도 따로 비교하면 앞 한글의 PDF−RHWP x 차이는 약 +0.63~+0.73px,
뒤 한글은 약 +0.04~+0.15px로 줄어든다(`issue8040-title-origins-all.log`). PDF의
쉼표는 내장 CID 글꼴 `F4`의 폭 333/1000em, 크기 83·장평 92.941177%로 기록돼 있고,
현재 추적은 원본 영문 슬롯 `한양신명조`의 Native OTF 폭 235/1000em을 사용한다.
시스템 `ENSMJ.otf`의 쉼표 hmtx도 235/1000em이다. 원본 문단은 가운데 정렬이므로
기호 폭 차이가 줄의 양쪽 원점 차이에 관여할 가능성이 있다. 다만 한컴 출력 환경의
글꼴 공급 차이와 저장 줄의 측정·배치 경로를 추가로 확인하기 전에는 특정 글꼴 치환이나
폭 변경을 공통 계약으로 확정하지 않는다. PDF 명령은 `issue8040-title-pdf-operators.log`다.

#8040 원본 1쪽의 현재 Native 후보를 독립 Print와 같은 배율로 다시 출력하고 전체
review·overlay를 생성했다. 로그 `issue8040-candidate-review.log`는 종료 코드 0이다.
이미지는 `issue8040-candidate-review/issue8040-current/review/review_001.png`와
`issue8040-candidate-review/issue8040-current/overlay/overlay_001.png`다. 이 이미지는
정밀 수치 하락이 남은 후보의 직접 판독 자료이며 기준 복구 완료 증거가 아니다.

GIF 자르기가 이미지 전체 범위 밖일 때 정규화 결과가 빈 선택이 될 수 있다. SVG의
`render_image_node_with_sampling`과 Native의 `image_conv`는 빈 선택을 명시적으로
건너뛰지만 Canvas의 `draw_image_with_fill_mode`에는 같은 명시 조건이 보이지 않는다.
이는 코드 검토에서 발견한 검증 대상이며 실제 한컴 Print의 빈 영역 계약과 Canvas 실행
결과는 아직 확인하지 않았다. 현재 WASM 빌드에 사용한 소스 해시를 유지하고 있으며,
독립 기준과 실행 결과를 확인한 뒤 필요한 공통 처리를 판단한다.

위 경계 확인을 위해 #7905 원본의 GIF 선택을 원본 영역 밖 `(50000, 0, 55000, 6000)`으로
수정한 진단 HWPX를 한컴 2020 MCP에서 HWP로 저장한 뒤 PrintToPDFEx 인쇄 PDF를 받았다.
HWP를 재파싱한 결과 해당 선택 좌표와 비어 있는 `imgDim`이 보존됐다. 3쪽 Native와
Print는 이미지 영역이 모두 비어 있고, raw 2px 실루엣은 100%다. 검토 이미지는
`issue7905-gif-empty-native/gif-empty/review/review_003.png`와
`issue7905-gif-empty-native/gif-empty/overlay/overlay_003.png`, MCP 저장·인쇄 기록은
`issue7905-gif-empty-selection-hwp-download.json`과
`issue7905-gif-empty-selection-pdf-download.json`이다. WASM 실행 결과는 아직 없다.
같은 HWP와 Print PDF의 3쪽 WASM 검증은 최적화 빌드와 선행 여섯 사례의 대조가 종료된
뒤 시작하도록 연결했다. 실행 로그는 `issue7905-gif-empty-wasm.log`이며 빌드 실패나
소스 해시 변경 시 자동으로 중단한다.

빈 선택 WASM 대조도 종료 코드 0이며 해당 3쪽 raw는 100%다. 독립 Print와 직접 비교해
선택된 이미지 영역이 비어 있는 것을 확인했다. Canvas에 명시적 빈 선택 분기가 없다는
코드 관찰만으로 결함이라고 결론 내리지 않으며, 확인한 입력에 대해 동작 보정은 추가하지
않는다. 증거는 `issue7905-gif-empty-wasm/gif-empty/review/review_003.png`와
`issue7905-gif-empty-wasm/gif-empty/overlay/overlay_003.png`다.
