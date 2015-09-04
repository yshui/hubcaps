
use std::io::Result;
use self::super::Github;
use rep::Label;
use deployments::Deployments;
use labels::Labels;
use pullrequests::PullRequests;
use rustc_serialize::json;
use issues::{IssueRef, Issues};

pub struct Repository<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> Repository<'a> {
  pub fn new(
    github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Repository<'a> {
    Repository {
      github: github,
      owner: owner,
      repo: repo
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}{}", self.owner, self.repo, more)
  }

  /// get a list of labels associated with this repository ref
  pub fn labels(&self) -> Labels {
    Labels::new(self.github, self.owner, self.repo)
  }

  /// get a list of deployments associated with this repository ref
  pub fn deployments(&self) -> Deployments {
    Deployments::new(self.github, self.owner, self.repo)
  }

  /// get a list of pulls associated with this repository ref
  pub fn pulls(&self) -> PullRequests {
    PullRequests::new(self.github, self.owner, self.repo)
  }

  /// get a reference to a specific github issue associated with this repoistory ref
  pub fn issue(&self, number: &'static i64) -> IssueRef {
    IssueRef::new(self.github, self.owner, self.repo, number)
  }

  pub fn issues(&self) -> Issues {
    Issues::new(self.github, self.owner, self.repo)
  }
}
