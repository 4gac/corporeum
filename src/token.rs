use crate::schema::Token;

impl Token {
    pub(crate) fn new(id: u32, token: &str) -> Self {
        Self {
            id,
            form: token.to_string(),
            lemma: None,
            upos: None,
            xpos: None,
            feats: None,
            head: None,
            deprel: None,
            deps: None,
            misc: None,
        }
    }

    pub fn get_form(&self) -> &str {
        &self.form
    }

    pub fn get_form_mut(&mut self) -> &mut String {
        &mut self.form
    }

    pub fn get_lemma(&self) -> Option<&str> {
        self.lemma.as_deref()
    }

    pub fn set_lemma(&mut self, lemma: &str) {
        self.lemma = Some(lemma.to_string());
    }

    pub fn remove_lemma(&mut self) {
        self.lemma = None;
    }

    pub fn get_upos(&self) -> Option<&str> {
        self.upos.as_deref()
    }

    pub fn set_upos(&mut self, upos: &str) {
        self.upos = Some(upos.to_string());
    }

    pub fn remove_upos(&mut self) {
        self.upos = None;
    }

    pub fn get_xpos(&self) -> Option<&str> {
        self.xpos.as_deref()
    }

    pub fn set_xpos(&mut self, xpos: &str) {
        self.xpos = Some(xpos.to_string());
    }

    pub fn remove_xpos(&mut self) {
        self.xpos = None;
    }

    pub fn get_feats(&self) -> Option<&str> {
        self.feats.as_deref()
    }

    pub fn set_feats(&mut self, feats: &str) {
        self.feats = Some(feats.to_string());
    }

    pub fn remove_feats(&mut self) {
        self.feats = None;
    }

    pub fn get_head(&self) -> Option<&str> {
        self.head.as_deref()
    }

    pub fn set_head(&mut self, head: &str) {
        self.head = Some(head.to_string());
    }

    pub fn remove_head(&mut self) {
        self.head = None;
    }

    pub fn get_deprel(&self) -> Option<&str> {
        self.deprel.as_deref()
    }

    pub fn set_deprel(&mut self, deprel: &str) {
        self.deprel = Some(deprel.to_string());
    }

    pub fn remove_deprel(&mut self) {
        self.deprel = None;
    }

    pub fn get_deps(&self) -> Option<&str> {
        self.deps.as_deref()
    }

    pub fn set_deps(&mut self, deps: &str) {
        self.deps = Some(deps.to_string());
    }

    pub fn remove_deps(&mut self) {
        self.deps = None;
    }

    pub fn get_misc(&self) -> Option<&str> {
        self.misc.as_deref()
    }

    pub fn set_misc(&mut self, misc: &str) {
        self.misc = Some(misc.to_string());
    }

    pub fn remove_misc(&mut self) {
        self.misc = None;
    }
}
