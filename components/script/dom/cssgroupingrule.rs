/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSGroupingRuleBinding;
use dom::bindings::codegen::Bindings::CSSGroupingRuleBinding::CSSGroupingRuleMethods;
use dom::bindings::codegen::Bindings::CSSRuleBinding::CSSRuleBinding::CSSRuleMethods;
use dom::bindings::error::{ErrorResult, Fallible};
use dom::bindings::inheritance::Castable;
use dom::bindings::js::{JS, MutNullableHeap, Root};
use dom::bindings::reflector::{Reflectable, reflect_dom_object};
use dom::bindings::str::DOMString;
use dom::cssrule::CSSRule;
use dom::cssrulelist::{CSSRuleList, RulesSource};
use dom::cssstylesheet::CSSStyleSheet;
use dom::window::Window;
use style::stylesheets::CssRules as StyleCssRules;

#[dom_struct]
pub struct CSSGroupingRule {
    cssrule: CSSRule,
    #[ignore_heap_size_of = "Arc"]
    rules: StyleCssRules,
    rulelist: MutNullableHeap<JS<CSSRuleList>>,
}

impl CSSGroupingRule {
    pub fn new_inherited(parent: Option<&CSSStyleSheet>,
                         rules: StyleCssRules) -> CSSGroupingRule {
        CSSGroupingRule {
            cssrule: CSSRule::new_inherited(parent),
            rules: rules,
            rulelist: MutNullableHeap::new(None),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(window: &Window, parent: Option<&CSSStyleSheet>, rules: StyleCssRules) -> Root<CSSGroupingRule> {
        reflect_dom_object(box CSSGroupingRule::new_inherited(parent, rules),
                           window,
                           CSSGroupingRuleBinding::Wrap)
    }

    fn rulelist(&self) -> Root<CSSRuleList> {
        let sheet = self.upcast::<CSSRule>().GetParentStyleSheet();
        let sheet = sheet.as_ref().map(|s| &**s);
        self.rulelist.or_init(|| CSSRuleList::new(self.global().as_window(),
                                                  sheet,
                                                  RulesSource::Rules(self.rules.clone())))
    }
}

impl CSSGroupingRuleMethods for CSSGroupingRule {
    // https://drafts.csswg.org/cssom/#dom-cssgroupingrule-cssrules
    fn CssRules(&self) -> Root<CSSRuleList> {
        // XXXManishearth check origin clean flag
        self.rulelist()
    }

    // https://drafts.csswg.org/cssom/#dom-cssgroupingrule-insertrule
    fn InsertRule(&self, rule: DOMString, index: u32) -> Fallible<u32> {
        self.rulelist().insert_rule(&rule, index, /* nested */ true)
    }

    // https://drafts.csswg.org/cssom/#dom-cssgroupingrule-deleterule
    fn DeleteRule(&self, index: u32) -> ErrorResult {
        self.rulelist().remove_rule(index)
    }
}
