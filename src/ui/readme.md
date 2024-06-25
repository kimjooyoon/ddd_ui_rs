## UI

목적: ***사용자가 서비스를 쉽게 사용한다***

### 용어 설명

- atom: 눈에 보이는 최소한의 요소를 가지며, theme(ux, 통일성)가 포함된 객체
- molecule: 1개 이상의 atom 을 가지며, logic(function, 기능성)이 포함된 객체
- organism: 1개 이상의 molecule 을 가지며, aggregate(business, 상품성)가 포함된 객체
- template: 1개 이상의 organism 을 가지며, detail-management(layout, 관리성)가 포함된 객체
- page: 1개 이상의 template 를 가지며, state(user-context, 상태)가 포함된 객체

### UI 객체

### UI 객체 목록

1. Atom
    - Button: 사용자 인터페이스에서 가장 기본적인 요소로, 클릭하여 액션을 실행합니다. (예: 제출 버튼, 취소 버튼)
    - Input Field: 사용자로부터 데이터를 입력받는 필드입니다. (예: 텍스트 입력, 비밀번호 입력)
    - Label: 텍스트로 정보를 제공하거나 입력 필드에 대한 설명을 제공하는 요소입니다.
    - Icon: 시각적 정보를 제공하거나 액션을 나타내는 작은 그래픽 요소입니다. (예: 검색 아이콘, 알림 아이콘)
    - Checkbox: 사용자가 선택할 수 있는 작은 상자입니다.
    - Radio Button: 사용자가 하나의 옵션만 선택할 수 있도록 하는 요소입니다.
    - Image: 그래픽이나 사진을 보여주는 요소입니다.
    - Link: 다른 페이지나 리소스로 이동할 수 있는 하이퍼링크 요소입니다.
2. Molecule
    - Form Group: 여러 개의 Input Field와 Label을 조합한 요소입니다. (예: 로그인 폼, 회원가입 폼)
    - Navigation Menu: 여러 개의 Link와 Icon을 조합하여 사용자가 페이지를 탐색할 수 있도록 하는 요소입니다.
    - Card: 제목, 이미지, 텍스트, 버튼 등 여러 atom을 포함하여 정보를 표시하는 요소입니다.
    - Modal: 팝업 창 형태로 사용자에게 추가 정보를 제공하거나 액션을 요구하는 요소입니다.
3. Organism
    - Header: 로고, Navigation Menu, 검색 창 등으로 구성된 페이지 상단 요소입니다.
    - Footer: 저작권 정보, 추가 링크, 연락처 정보 등을 포함하는 페이지 하단 요소입니다.
    - Product List: 여러 개의 Card를 포함하여 상품 정보를 나열하는 요소입니다.
    - Sidebar: 추가 Navigation Menu, 광고, 유틸리티 링크 등을 포함하는 페이지 측면 요소입니다.
4. Template
    - Main Layout: Header, Footer, Sidebar, Content Area 등을 포함하여 페이지의 기본 구조를 정의하는 요소입니다.
    - Dashboard Layout: 다양한 통계 정보와 그래프를 보여주는 organism을 포함하는 레이아웃입니다.
    - E-commerce Template: Product List, 카트, 추천 상품 등을 포함하는 쇼핑몰 페이지 레이아웃입니다.
5. Page
    - Home Page: Main Layout을 기반으로 주요 콘텐츠, 추천 상품, 최신 뉴스 등을 보여주는 페이지입니다.
    - Product Detail Page: 특정 상품에 대한 상세 정보를 제공하는 페이지로, 이미지, 설명, 리뷰 등을 포함합니다.
    - User Profile Page: 사용자 정보와 활동 내역을 보여주는 페이지입니다.
    - Checkout Page: 구매 과정의 최종 단계를 진행하는 페이지로, 결제 정보 입력 및 확인을 포함합니다.
