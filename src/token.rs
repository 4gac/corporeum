use crate::schema::Token;

impl Token {
    pub fn new(id: u32, token: &str) -> Token {
        Token {
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

    pub fn form(&self) -> &str {
        &self.form
    }

    pub fn form_mut(&mut self) -> &mut String {
        &mut self.form
    }

    pub fn lemma(&self) -> Option<&str> {
        self.lemma.as_ref().map(|x| x.as_str())
    }

    pub fn set_lemma(&mut self, lemma: &str) {
        self.lemma = Some(lemma.to_string());
    }

    pub fn remove_lemma(&mut self) {
        self.lemma = None;
    }

    pub fn upos(&self) -> Option<&str> {
        self.upos.as_ref().map(|x| x.as_str())
    }

    pub fn set_upos(&mut self, upos: &str) {
        self.upos = Some(upos.to_string());
    }

    pub fn remove_upos(&mut self) {
        self.upos = None;
    }
    pub fn xpos(&self) -> Option<&str> {
        self.xpos.as_ref().map(|x| x.as_str())
    }

    pub fn set_xpos(&mut self, xpos: &str) {
        self.xpos = Some(xpos.to_string());
    }

    pub fn remove_xpos(&mut self) {
        self.xpos = None;
    }

    pub fn feats(&self) -> Option<&str> {
        self.feats.as_ref().map(|x| x.as_str())
    }

    pub fn set_feats(&mut self, feats: &str) {
        self.feats = Some(feats.to_string());
    }

    pub fn remove_feats(&mut self) {
        self.feats = None;
    }

    pub fn head(&self) -> Option<&str> {
        self.head.as_ref().map(|x| x.as_str())
    }

    pub fn set_head(&mut self, head: &str) {
        self.head = Some(head.to_string());
    }

    pub fn remove_head(&mut self) {
        self.head = None;
    }

    pub fn deprel(&self) -> Option<&str> {
        self.deprel.as_ref().map(|x| x.as_str())
    }

    pub fn set_deprel(&mut self, deprel: &str) {
        self.deprel = Some(deprel.to_string());
    }

    pub fn remove_deprel(&mut self) {
        self.deprel = None;
    }

    pub fn deps(&self) -> Option<&str> {
        self.deps.as_ref().map(|x| x.as_str())
    }

    pub fn set_deps(&mut self, deps: &str) {
        self.deps = Some(deps.to_string());
    }

    pub fn remove_deps(&mut self) {
        self.deps = None;
    }

    pub fn misc(&self) -> Option<&str> {
        self.misc.as_ref().map(|x| x.as_str())
    }

    pub fn set_misc(&mut self, misc: &str) {
        self.misc = Some(misc.to_string());
    }

    pub fn remove_misc(&mut self) {
        self.misc = None;
    }
}
