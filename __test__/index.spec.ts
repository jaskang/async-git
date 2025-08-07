import test from 'ava'

import { gitStatus, gitStatusWithFetch } from '../index.js'

const modules: any[] = [
    {
        "name": "公线旧仓库",
        "owner": "weberli(李少鹏)",
        "kind": "base",
        "path": "app/pages/insure/main",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-main.git"
    },
    {
        "name": "社保个账",
        "owner": "dalyhuang(黄东煜)",
        "path": "app/pages/insure/social",
        "repo": "git@e.coding.inwesure.com:wesure/g014/social.git"
    },
    {
        "name": "车险",
        "owner": "xutaochen(陈旭涛)",
        "kind": "car",
        "path": "app/pages/insure/carReinsure",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-carReinsure.git"
    },
    {
        "name": "查违章",
        "owner": "xutaochen(陈旭涛)",
        "kind": "car",
        "path": "app/pages/insure/checkIllegal",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-checkIllegal.git"
    },
    {
        "name": "驾乘意外",
        "owner": "xutaochen(陈旭涛)",
        "kind": "pa",
        "path": "app/pages/insure/drivingInsure",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-drivingAccidentInsure.git"
    },
    {
        "name": "旅行险",
        "owner": "carldmzhao(赵熙明)",
        "path": "app/pages/insure/travelInsure",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure_miniapp_travelInsure.git"
    },
    {
        "name": "旧组件库",
        "owner": "laelli(李锂)",
        "kind": "base",
        "path": "app/base/components",
        "repo": "git@e.coding.inwesure.com:wesure/g014/ws-components.git"
    },
    {
        "name": "通用框架",
        "owner": "sherryswang(王舒展)",
        "path": "app/pages/template/base",
        "kind": "template-base",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-template-base.git"
    },
    {
        "name": "基础能力",
        "owner": "weberli(李少鹏)",
        "kind": "base",
        "path": "app/pages/base",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-page-base.git"
    },
    {
        "name": "通用框架",
        "owner": "sherryswang(王舒展)",
        "path": "app/pages/common",
        "kind": "template-base",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-page-common.git"
    },
    {
        "name": "活动仓库",
        "owner": "ninawwwu(吴桔红)",
        "path": "app/pages/promotion",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-page-promotion.git"
    },
    {
        "name": "健康险主页",
        "owner": "raymondye(叶瑞敏)",
        "path": "app/pages/product/PH00",
        "repo": "git@e.coding.inwesure.com:wesure/g014/PH00.git"
    },
    {
        "name": "小程序页卡",
        "owner": "weberli(李少鹏)",
        "kind": "base",
        "path": "app/pages/tabbar",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-page-tabbar.git"
    },
    {
        "name": "个人中心分包",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/insure/personal",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-personal.git"
    },
    {
        "name": "新组件库",
        "owner": "weberli(李少鹏)",
        "kind": "base",
        "path": "app/components",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-components.git"
    },
    {
        "name": "main分包",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/product/main",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-main.git"
    },
    {
        "name": "发现页卡分包",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/product/discovery",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-discovery.git"
    },
    {
        "name": "二维码收款安全险",
        "owner": "baggioschen(陈志毅)",
        "path": "app/pages/product/PT03",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-PT03.git"
    },
    {
        "name": "创新险总包",
        "owner": "carldmzhao(赵熙明)",
        "path": "app/pages/insure/innovate",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-insure-innovate.git"
    },
    {
        "name": "微管家",
        "owner": "kevinhyang(杨凯)",
        "path": "app/pages/wecare",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-wecare.git"
    },
    {
        "name": "步数王者",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/longTerm/activityIDstepKing",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-promotion-stepKing.git"
    },
    {
        "name": "wedrive",
        "owner": "xutaochen(陈旭涛)",
        "path": "app/pages/wedrive",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-wedrive.git"
    },
    {
        "name": "碎屏险",
        "owner": "ellisli(李弼林)",
        "path": "app/pages/product/PT12",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-PT12.git"
    },
    {
        "name": "寿险频道",
        "owner": "vczheng(郑鸿生)",
        "path": "app/pages/product/LL00",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-LL00.git"
    },
    {
        "name": "年金",
        "owner": "caleliu(刘惠龙)",
        "path": "app/pages/template/annuity",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-template-annuity.git"
    },
    {
        "name": "意外险频道页",
        "owner": "v_lngcchen(陈龙)",
        "path": "app/pages/product/PA00",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-pa00.git"
    },
    {
        "name": "产品详情页投放版",
        "owner": "jiluanchen(陈集銮)",
        "path": "app/pages/ads/launchProduct",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-ads-wrapper.git"
    },
    {
        "name": "营销活动新分包",
        "owner": "v_janlwang(王建林)",
        "path": "app/pages/activity",
        "repo": "git@e.coding.inwesure.com:wesure/g014/activity.git"
    },
    {
        "name": "惠民保项目分包",
        "owner": "dalyhuang(黄东煜)",
        "path": "app/pages/longTerm/benefitThePeople",
        "repo": "git@e.coding.inwesure.com:wesure/g014/benefit-the-people.git"
    },
    {
        "name": "多X多保项目分包",
        "owner": "v_lngcchen(陈龙)",
        "path": "app/pages/product/morex",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-morex.git"
    },
    {
        "name": "摩托车险分包",
        "owner": "xutaochen(陈旭涛)",
        "path": "app/pages/insure/motorcycle",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-motorcycle.git"
    },
    {
        "name": "赠险分包",
        "owner": "raymondye(叶瑞敏)",
        "path": "app/pages/product/gift",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-gift-insure.git"
    },
    {
        "name": "权益",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/product/benefits",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-benefits.git"
    },
    {
        "name": "理赔分包",
        "owner": "guolicheng(程振国)",
        "kind": "claim",
        "path": "app/pages/claim",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-claim.git"
    },
    {
        "name": "就医理赔助手分包",
        "owner": "guolicheng(程振国)",
        "kind": "misc",
        "path": "app/pages/misc/claimWxAssistant",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-claimWxAssistant.git"
    },
    {
        "name": "保单管家子包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/policySteward",
        "repo": "git@e.coding.inwesure.com:wesure/g014/policy-steward.git"
    },
    {
        "name": "非车专项成交",
        "owner": "vczheng(郑鸿生)",
        "kind": "operation",
        "path": "app/pages/operation/noncar",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-operation-noncar.git"
    },
    {
        "name": "赠险广告投放独立子包",
        "owner": "ninawwwu(吴桔红)",
        "path": "app/pages/ads/giftInsure",
        "repo": "git@e.coding.inwesure.com:wesure/g014/ads-gift-insure.git"
    },
    {
        "name": "微管家独立子包",
        "owner": "kevinhyang(杨凯)",
        "path": "app/pages/ads/wecare",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-ads-wecare.git"
    },
    {
        "name": "群投保项目",
        "owner": "qiushzhang(张立宇)",
        "kind": "product",
        "path": "app/pages/product/groupInsurance",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-groupinsurance.git"
    },
    {
        "name": "财产险分包",
        "owner": "ellisli(李弼林)",
        "path": "app/pages/product/property",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-property.git"
    },
    {
        "name": "年金类产品页面",
        "owner": "vczheng(郑鸿生)",
        "path": "app/pages/product/annuity",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-annuity.git"
    },
    {
        "name": "新通用投保流程",
        "owner": "jiluanchen(陈集銮)",
        "path": "app/pages/product/common",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-common.git"
    },
    {
        "name": "新续保流程",
        "owner": "jiluanchen(陈集銮)",
        "path": "app/pages/product/renewal",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-renewal.git"
    },
    {
        "name": "年金和通用投保流程公共分包",
        "owner": "jiluanchen(陈集銮)",
        "path": "app/pages/product/shared",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-shared.git"
    },
    {
        "name": "product下异步分包公共组件库",
        "owner": "jiluanchen(陈集銮)",
        "path": "app/pages/product/sharedComponents",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-sharedComponents.git"
    },
    {
        "name": "专属养老金分包",
        "owner": "vczheng(郑鸿生)",
        "path": "app/pages/product/exclusivePension",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-exclusivePension.git"
    },
    {
        "name": "工具渠道分包",
        "owner": "vczheng(郑鸿生)",
        "path": "app/pages/tools/channel",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-tools-channel.git"
    },
    {
        "name": "工具保险大全分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/insuranceTools",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-insurance-tools.git"
    },
    {
        "name": "工具保险方案分包",
        "owner": "guolicheng(程振国)",
        "path": "app/pages/tools/insuranceProgram",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-tools-program.git"
    },
    {
        "name": "工具风险测评分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/riskEvaluate",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-risk-evaluate.git"
    },
    {
        "name": "工具疾病投保分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/diseaseInsure",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-disease-insure.git"
    },
    {
        "name": "工具保险购物车分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/shoppingCart",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-shopping-cart.git"
    },
    {
        "name": "工具税优税延分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/taxDiscount",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-tax-discount.git"
    },
    {
        "name": "工具泛保险分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/comprehensive",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-tools-comprehensive.git"
    },
    {
        "name": "工具保单解读分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/policyInterpretation",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-policy-interpretation.git"
    },
    {
        "name": "工具保单检视分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/policyReview",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-policy-review.git"
    },
    {
        "name": "工具公共能力分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/tools/common",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-tools-common.git"
    },
    {
        "name": "工具投放独立分包",
        "owner": "zehuangzhou(周泽煌)",
        "path": "app/pages/ads/tools",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-ads-tools.git"
    },
    {
        "name": "生日礼活动分包",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/longTerm/birthdayGift",
        "kind": "birthdayGift",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-activity-birthdayGift.git"
    },
    {
        "name": "微保直播",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/product/live",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-live.git"
    },
    {
        "name": "少儿权益",
        "owner": "jaunehuang(黄丽)",
        "path": "app/pages/main/childrenBenefits",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-children-benefits.git"
    },
    {
        "name": "外部合作个险定制",
        "owner": "dalyhuang(黄东煜)",
        "path": "app/pages/product/partner",
        "repo": "git@e.coding.inwesure.com:wesure/g014/product-partner.git"
    },
    {
        "name": "长险年金投保",
        "owner": "vczheng(郑鸿生)",
        "path": "app/pages/product/annuityInsurance",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-annuityInsurance.git"
    },
    {
        "name": "用户运营相关",
        "owner": "leopeixie(谢培)",
        "path": "app/pages/misc/up",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-up.git"
    },
    {
        "name": "管顾客服异步组件分包",
        "owner": "leopeixie(谢培)",
        "path": "app/pages/misc/asyncWecareComponent",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-asyncWecareComponent.git"
    },
    {
        "name": "月报v2",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/main/monthlyReportV2",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-main-monthlyReportV2.git"
    },
    {
        "name": "综合服务页面",
        "owner": "v_janlwang(王建林)",
        "path": "app/pages/main/integratedService",
        "repo": "git@e.coding.inwesure.com:wesure/g014/main-integrated-service.git"
    },
    {
        "name": "微信支付人身险合作产品",
        "owner": "ninawwwu(吴桔红)",
        "path": "app/pages/product/wxPayHealth",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-wxPayHealth.git"
    },
    {
        "name": "工具风险大数据",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/tools/diseaseRisk",
        "kind": "tools",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-disease-risk.git"
    },
    {
        "name": "养老险分包",
        "owner": "vczheng(郑鸿生)",
        "path": "app/pages/service/pensionInsurance",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-pension-insurance.git"
    },
    {
        "name": "用户手册分包",
        "owner": "v_jinlluo(罗进)",
        "path": "app/pages/main/userManual",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-userManual.git"
    },
    {
        "name": "小程序ECharts组件异步分包",
        "owner": "corinnaliu(刘倩)",
        "path": "app/pages/misc/asyncEcharts",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-asyncEcharts.git"
    },
    {
        "name": "智能客服",
        "owner": "kevinhyang(杨凯)",
        "path": "app/pages/misc/wecare",
        "kind": "misc",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-wecare-robot.git"
    },
    {
        "name": "会员外投独立子包",
        "owner": "v_jinlluo(罗进)",
        "path": "app/pages/ads/memberBenefits",
        "kind": "memberBenefits",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-ads-benefits.git"
    },
    {
        "name": "保单相关",
        "owner": "jaunehuang(黄丽)",
        "path": "app/pages/product/policy",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-product-policy.git"
    },
    {
        "name": "频道页分包",
        "owner": "checkcai(蔡海真)",
        "path": "app/pages/channel",
        "kind": "channel",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-channel.git"
    },
    {
        "name": "组件共享包",
        "owner": "checkcai(蔡海真)",
        "path": "app/shared",
        "repo": "git@e.coding.inwesure.com:wesure/g014/shared.git"
    },
    {
        "name": "折扣专区",
        "owner": "v_jinlluo(罗进)",
        "path": "app/pages/longTerm/discountZone",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-discount-zone.git"
    },
    {
        "name": "发现-我的账户卡片",
        "owner": "changerhe(何庆畅)",
        "path": "app/pages/ads/accountCard",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-account-card.git"
    },
    {
        "name": "邀请评价有礼",
        "owner": "weberli(李少鹏)",
        "path": "app/pages/longTerm/inviteForEvaluation",
        "repo": "git@e.coding.inwesure.com:wesure/g014/miniapp-activity-inviteForEvaluation.git"
    },
    {
        "name": "理赔服务分包",
        "owner": "caleliu(刘惠龙)",
        "path": "app/pages/service/claims",
        "kind": "claim",
        "repo": "git@e.coding.inwesure.com:wesure/g014/wesure-miniapp-claimService.git"
    }
]
test('sync function from native code', async (t) => {
  const dirs = modules.map((m) => `/Users/jaskang/code/test/${m.path}`)
  const start = Date.now()
  await gitStatusWithFetch(dirs)
    .then((results) => {
      console.log('lines:', results)
      const end = Date.now()
      console.log(`gitStatusWithFetch took ${end - start} ms`)
    })
    .catch((error) => {
      console.error('error:', error)
      t.fail('Expected the command to succeed, but it failed.')
    })
  t.is(true, true, 'This test is a placeholder and should be replaced with actual assertions.')
})
