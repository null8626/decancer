use super::{BracketPair, Class, Level, OpeningBracket};
use crate::Error;
use std::{
  cmp::{max, min},
  ops::{Index, IndexMut, Range},
};

#[derive(PartialEq)]
pub(crate) enum OverrideStatus {
  Neutral,
  RTL,
  LTR,
  Isolate,
}

struct Status {
  level: Level,
  status: OverrideStatus,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct IsolatingRunSequence {
  pub(crate) runs: Vec<Range<usize>>,
  pub(crate) start_class: Class,
  pub(crate) end_class: Class,
}

impl IsolatingRunSequence {
  pub(crate) fn resolve_implicit_weak(&self, text: &str, processing_classes: &mut [Class]) {
    let mut prev_class_before_w4 = self.start_class;
    let mut prev_class_before_w5 = self.start_class;

    let mut last_strong_is_al = false;

    let mut et_run_indices = Vec::new();
    let mut bn_run_indices = Vec::new();

    for (run_index, level_run) in self.runs.iter().enumerate() {
      for i in level_run.clone() {
        if processing_classes[i] == Class::BN {
          bn_run_indices.push(i);
          continue;
        }

        let w2_processing_class = processing_classes[i];

        match processing_classes[i] {
          Class::EN => {
            if last_strong_is_al {
              processing_classes[i] = Class::AN;
            }
          },
          Class::AL => processing_classes[i] = Class::R,
          _ => {},
        }

        match w2_processing_class {
          Class::L | Class::R => last_strong_is_al = false,
          Class::AL => last_strong_is_al = true,
          _ => {},
        }

        let class_before_w456 = processing_classes[i];

        match processing_classes[i] {
          Class::EN => {
            while let Some(j) = et_run_indices.pop() {
              processing_classes[j] = Class::EN;
            }
          },

          Class::ES | Class::CS => {
            if let Some(character) = text.get(i..).and_then(|x| x.chars().next()) {
              let char_len = character.len_utf8();

              let mut next_class = self
                .iter_forwards_from(i + char_len, run_index)
                .map(|j| processing_classes[j])
                .find(|x| !x.removed_by_x9())
                .unwrap_or(self.end_class);

              if next_class == Class::EN && last_strong_is_al {
                next_class = Class::AN;
              }

              processing_classes[i] =
                match (prev_class_before_w4, processing_classes[i], next_class) {
                  (Class::EN, Class::ES | Class::CS, Class::EN) => Class::EN,
                  (Class::AN, Class::CS, Class::AN) => Class::AN,
                  _ => Class::ON,
                };

              if processing_classes[i] == Class::ON {
                for idx in self.iter_backwards_from(i, run_index) {
                  let class = &mut processing_classes[idx];
                  if *class != Class::BN {
                    break;
                  }

                  *class = Class::ON;
                }

                for idx in self.iter_forwards_from(i + char_len, run_index) {
                  let class = &mut processing_classes[idx];
                  if *class != Class::BN {
                    break;
                  }

                  *class = Class::ON;
                }
              }
            } else {
              processing_classes[i] = processing_classes[i - 1];
            }
          },

          Class::ET => match prev_class_before_w5 {
            Class::EN => processing_classes[i] = Class::EN,
            _ => {
              et_run_indices.extend(&bn_run_indices);
              et_run_indices.push(i);
            },
          },
          _ => {},
        }

        bn_run_indices.clear();
        prev_class_before_w5 = processing_classes[i];

        if prev_class_before_w5 != Class::ET {
          while let Some(j) = et_run_indices.pop() {
            processing_classes[j] = Class::ON;
          }
        }

        prev_class_before_w4 = class_before_w456;
      }
    }

    while let Some(j) = et_run_indices.pop() {
      processing_classes[j] = Class::ON;
    }

    let mut last_strong_is_l = self.start_class == Class::L;

    for i in self.runs.iter().cloned().flatten() {
      match processing_classes[i] {
        Class::EN if last_strong_is_l => processing_classes[i] = Class::L,
        Class::L => last_strong_is_l = true,
        Class::R | Class::AL => last_strong_is_l = false,
        _ => {},
      }
    }
  }

  pub(crate) fn identify_bracket_pairs(
    &self,
    text: &str,
    original_classes: &[Class],
    bracket_pairs: &mut Vec<BracketPair>,
  ) {
    let mut stack = Vec::new();

    for (run_index, level_run) in self.runs.iter().enumerate() {
      for (i, ch) in text[level_run.clone()].char_indices() {
        let actual_index = level_run.start + i;

        if original_classes[actual_index] != Class::ON {
          continue;
        }

        if let Some(matched) = OpeningBracket::new(ch as _) {
          if matched.is_open {
            if stack.len() >= 63 {
              break;
            }

            stack.push((matched.opening, actual_index, run_index));
          } else if let Some((stack_index, element)) = stack
            .iter()
            .enumerate()
            .rev()
            .find(|(_, element)| element.0 == matched.opening)
          {
            bracket_pairs.push(BracketPair {
              start: element.1,
              end: actual_index,
              start_run: element.2,
              end_run: run_index,
            });

            stack.truncate(stack_index);
          }
        }
      }
    }

    bracket_pairs.sort_by_key(|r| r.start);
  }

  pub(crate) fn resolve_implicit_neutral(
    &self,
    text: &str,
    processing_classes: &mut [Class],
    levels: &[Level],
  ) {
    let e = levels[self.runs[0].start].class();

    let not_e = if e == Class::L { Class::R } else { Class::L };
    let mut bracket_pairs = Vec::new();

    self.identify_bracket_pairs(text, processing_classes, &mut bracket_pairs);

    for pair in bracket_pairs {
      let mut found_e = false;
      let mut found_not_e = false;
      let mut class_to_set = None;

      let start_char_len = text[pair.start..].chars().next().unwrap().len_utf8();

      for enclosed_i in self.iter_forwards_from(pair.start + start_char_len, pair.start_run) {
        if enclosed_i >= pair.end {
          break;
        }

        let class = processing_classes[enclosed_i];

        if class == e {
          found_e = true;
          break;
        } else if class == not_e {
          found_not_e = true;
        } else if matches!(class, Class::EN | Class::AN) {
          if e == Class::L {
            found_not_e = true;
          } else {
            found_e = true;
            break;
          }
        }
      }

      if found_e {
        class_to_set.replace(e);
      } else if found_not_e {
        let mut previous_strong = self
          .iter_backwards_from(pair.start, pair.start_run)
          .map(|i| processing_classes[i])
          .find(|class| matches!(class, Class::L | Class::R | Class::EN | Class::AN))
          .unwrap_or(self.start_class);

        if matches!(previous_strong, Class::EN | Class::AN) {
          previous_strong = Class::R;
        }

        class_to_set.replace(previous_strong);
      }

      if let Some(class_to_set) = class_to_set {
        let end_char_len = text[pair.end..].chars().next().unwrap().len_utf8();

        for class in
          (pair.start..pair.start + start_char_len).chain(pair.end..pair.end + end_char_len)
        {
          processing_classes[class] = class_to_set;
        }

        for idx in self.iter_backwards_from(pair.start, pair.start_run) {
          let class = &mut processing_classes[idx];

          if *class != Class::BN {
            break;
          }

          *class = class_to_set;
        }

        for idx in self.iter_forwards_from(pair.start + start_char_len, pair.start_run) {
          if processing_classes[idx] == Class::BN {
            processing_classes[idx] = class_to_set;
          } else {
            break;
          }
        }

        for idx in self.iter_forwards_from(pair.end + end_char_len, pair.end_run) {
          if processing_classes[idx] == Class::BN {
            processing_classes[idx] = class_to_set;
          } else {
            break;
          }
        }
      }
    }

    let mut indices = self.runs.iter().flat_map(Clone::clone);
    let mut prev_class = self.start_class;

    while let Some(mut i) = indices.next() {
      let mut ni_run = Vec::new();

      if processing_classes[i].is_neutral_or_isolate() || processing_classes[i] == Class::BN {
        ni_run.push(i);
        let mut next_class;

        loop {
          match indices.next() {
            Some(j) => {
              i = j;
              next_class = processing_classes[j];

              if next_class.is_neutral_or_isolate() || next_class == Class::BN {
                ni_run.push(i);
              } else {
                break;
              }
            },

            None => {
              next_class = self.end_class;
              break;
            },
          }
        }

        let new_class = match (prev_class, next_class) {
          (Class::L, Class::L) => Class::L,
          (Class::R | Class::AN | Class::EN, Class::R | Class::AN | Class::EN) => Class::R,
          _ => e,
        };

        while let Some(j) = ni_run.pop() {
          processing_classes[j] = new_class;
        }
      }

      prev_class = processing_classes[i];
    }
  }

  fn iter_forwards_from(
    &self,
    pos: usize,
    level_run_index: usize,
  ) -> impl Iterator<Item = usize> + '_ {
    let runs = &self.runs[level_run_index..];

    (pos..runs[0].end).chain(runs[1..].iter().flat_map(Clone::clone))
  }

  fn iter_backwards_from(
    &self,
    pos: usize,
    level_run_index: usize,
  ) -> impl Iterator<Item = usize> + '_ {
    (self.runs[level_run_index].start..pos).rev().chain(
      self.runs[..level_run_index]
        .iter()
        .rev()
        .flat_map(Clone::clone),
    )
  }
}

pub(crate) struct Paragraph {
  pub(crate) range: Range<usize>,
  pub(crate) level: Level,
  pub(crate) pure_ltr: bool,
  pub(crate) has_isolate_controls: bool,
}

impl Paragraph {
  #[inline(always)]
  pub(crate) fn sliced<'a, T: Index<Range<usize>> + ?Sized>(
    &'a self,
    slicable: &'a T,
  ) -> &'a <T as Index<Range<usize>>>::Output {
    &slicable[self.range.clone()]
  }

  #[inline(always)]
  pub(crate) fn sliced_mut<'a, T: IndexMut<Range<usize>> + ?Sized>(
    &'a self,
    slicable: &'a mut T,
  ) -> &'a mut <T as Index<Range<usize>>>::Output {
    &mut slicable[self.range.clone()]
  }

  pub(crate) fn visual_runs(
    &self,
    text: &str,
    original_classes: &[Class],
    levels: &[Level],
  ) -> Result<(Vec<Level>, Vec<Range<usize>>), Error> {
    let mut levels = Vec::from(levels);

    let mut reset_from: Option<usize> = Some(0);
    let mut reset_to: Option<usize> = None;
    let mut prev_level = self.level;

    for (i, c) in text.char_indices() {
      match original_classes[i] {
        Class::B | Class::S => {
          reset_to.replace(i + c.len_utf8());

          if reset_from.is_none() {
            reset_from.replace(i);
          }
        },

        Class::WS | Class::FSI | Class::LRI | Class::RLI | Class::PDI => {
          if reset_from.is_none() {
            reset_from.replace(i);
          }
        },

        Class::RLE | Class::LRE | Class::RLO | Class::LRO | Class::PDF | Class::BN => {
          if reset_from.is_none() {
            reset_from.replace(i);
          }

          levels[i] = prev_level;
        },

        _ => {
          reset_from = None;
        },
      }

      if let (Some(from), Some(to)) = (reset_from, reset_to) {
        for level in &mut levels[from..to] {
          *level = self.level;
        }

        reset_from = None;
        reset_to = None;
      }

      prev_level = levels[i];
    }

    if let Some(from) = reset_from {
      for level in &mut levels[from..] {
        *level = self.level;
      }
    }

    let mut runs = Vec::with_capacity(1);
    let mut start = self.range.start;
    let mut run_level = levels[start];
    let mut min_level = run_level;
    let mut max_level = run_level;

    for (i, &new_level) in levels
      .iter()
      .enumerate()
      .take(self.range.end)
      .skip(start + 1)
    {
      if new_level != run_level {
        runs.push(start..i);

        start = i;
        run_level = new_level;

        min_level = min(run_level, min_level);
        max_level = max(run_level, max_level);
      }
    }

    runs.push(start..self.range.end);

    let run_count = runs.len();
    min_level = min_level.new_lowest_ge_rtl()?;

    while max_level >= min_level {
      let mut seq_start = 0;

      while seq_start < run_count {
        if levels[runs[seq_start].start] < max_level {
          seq_start += 1;
          continue;
        }

        let mut seq_end = seq_start + 1;

        while seq_end < run_count && levels[runs[seq_end].start] >= max_level {
          seq_end += 1;
        }

        runs[seq_start..seq_end].reverse();
        seq_start = seq_end;
      }

      max_level.lower(1)?;
    }

    Ok((levels, runs))
  }

  pub(crate) fn compute_explicit(
    &self,
    input: &str,
    original_classes: &[Class],
    processing_classes: &mut [Class],
    levels: &mut [Level],
    runs: &mut Vec<Range<usize>>,
  ) -> Result<(), Error> {
    let mut stack = vec![Status {
      level: self.level,
      status: OverrideStatus::Neutral,
    }];

    let mut overflow_isolate_count = 0;
    let mut overflow_embedding_count = 0;
    let mut valid_isolate_count = 0;

    let mut current_run_level = Level::ltr();
    let mut current_run_start = 0;

    for (idx, character) in input.char_indices() {
      let current_class = original_classes[idx];

      match current_class {
        Class::RLE
        | Class::LRE
        | Class::RLO
        | Class::LRO
        | Class::RLI
        | Class::LRI
        | Class::FSI => {
          let Some(last) = stack.last() else {
            return Err(Error::MalformedOverrideStatusStack);
          };

          levels[idx] = last.level;

          let is_isolate = current_class.is_isolate();

          if is_isolate {
            match last.status {
              OverrideStatus::RTL => processing_classes[idx] = Class::R,
              OverrideStatus::LTR => processing_classes[idx] = Class::L,
              _ => {},
            }
          }

          let new_level = if current_class.is_rtl() {
            last.level.new_explicit_next_rtl()
          } else {
            last.level.new_explicit_next_ltr()
          };

          match (new_level, overflow_isolate_count, overflow_embedding_count) {
            (Ok(new_level), 0, 0) => {
              stack.push(Status {
                level: new_level,
                status: original_classes[idx].override_status(),
              });

              if is_isolate {
                valid_isolate_count += 1;
              } else {
                levels[idx] = new_level;
              }
            },

            _ => {
              if is_isolate {
                overflow_isolate_count += 1;
              } else if overflow_isolate_count == 0 {
                overflow_embedding_count += 1;
              }
            },
          }

          if !is_isolate {
            processing_classes[idx] = Class::BN;
          }
        },

        Class::PDI => {
          if overflow_isolate_count > 0 {
            overflow_isolate_count -= 1;
          } else if valid_isolate_count > 0 {
            overflow_embedding_count = 0;

            while !matches!(
              stack.pop(),
              None
                | Some(Status {
                  status: OverrideStatus::Isolate,
                  ..
                })
            ) {}

            valid_isolate_count -= 1;
          }

          let Some(last) = stack.last() else {
            return Err(Error::MalformedOverrideStatusStack);
          };

          levels[idx] = last.level;

          match last.status {
            OverrideStatus::RTL => processing_classes[idx] = Class::R,
            OverrideStatus::LTR => processing_classes[idx] = Class::L,
            _ => {},
          }
        },

        Class::PDF => {
          if overflow_isolate_count <= 0 {
            if overflow_embedding_count > 0 {
              overflow_embedding_count -= 1;
            } else {
              let Some(last) = stack.last() else {
                return Err(Error::MalformedOverrideStatusStack);
              };

              if stack.len() >= 2 && last.status != OverrideStatus::Isolate {
                stack.pop();
              }
            }
          }

          let Some(last) = stack.last() else {
            return Err(Error::MalformedOverrideStatusStack);
          };

          levels[idx] = last.level;
          processing_classes[idx] = Class::BN;
        },

        Class::B => {},

        _ => {
          let Some(last) = stack.last() else {
            return Err(Error::MalformedOverrideStatusStack);
          };

          levels[idx] = last.level;

          if current_class != Class::BN {
            match last.status {
              OverrideStatus::RTL => processing_classes[idx] = Class::R,
              OverrideStatus::LTR => processing_classes[idx] = Class::L,
              _ => {},
            }
          }
        },
      }

      for j in 1..character.len_utf8() {
        levels[idx + j] = levels[idx];
        processing_classes[idx + j] = processing_classes[idx];
      }

      if idx == 0 {
        current_run_level = levels[idx];
      } else if original_classes[idx].removed_by_x9() && levels[idx] != current_run_level {
        runs.push(current_run_start..idx);
        current_run_level = levels[idx];
        current_run_start = idx;
      }
    }

    if levels.len() > current_run_start {
      runs.push(current_run_start..levels.len());
    }

    Ok(())
  }

  pub(crate) fn isolating_run_sequences(
    &self,
    levels: &[Level],
    level_runs: &[Range<usize>],
    original_classes: &[Class],
    irs: &mut Vec<IsolatingRunSequence>,
  ) -> Result<(), Error> {
    if self.has_isolate_controls {
      let mut runs = Vec::new();

      if let Some(&(mut current_run_level)) = levels.first() {
        let mut current_run_start = 0;

        for i in 1..levels.len() {
          if !original_classes[i].removed_by_x9() && levels[i] != current_run_level {
            runs.push(current_run_start..i);

            current_run_level = levels[i];
            current_run_start = i;
          }
        }

        runs.push(current_run_start..levels.len());
      }

      let mut sequences = Vec::with_capacity(runs.len());
      let mut stack = vec![vec![]];

      for run in runs {
        let start_class = original_classes[run.start];
        let end_class = original_classes[run.clone()]
          .iter()
          .copied()
          .rev()
          .find(|x| !x.removed_by_x9())
          .unwrap_or(start_class);

        let mut sequence = if start_class == Class::PDI && stack.len() > 1 {
          stack.pop().unwrap()
        } else {
          Vec::with_capacity(1)
        };

        sequence.push(run);

        if end_class.is_isolate() {
          stack.push(sequence);
        } else {
          sequences.push(sequence);
        }
      }

      sequences.extend(stack.into_iter().rev().filter(|seq| !seq.is_empty()));

      irs.reserve_exact(sequences.len());

      for sequence in sequences {
        if sequence.is_empty() {
          return Err(Error::MalformedIsolatingRunSequence);
        }

        let sequence_start = sequence[0].start;
        let runs_len = sequence.len();
        let sequence_end = sequence[runs_len - 1].end;

        let mut result = IsolatingRunSequence {
          runs: sequence,
          start_class: Class::L,
          end_class: Class::L,
        };

        let sequence_level = levels[result
          .iter_forwards_from(sequence_start, 0)
          .find(|&i| !original_classes[i].removed_by_x9())
          .unwrap_or(sequence_start)];

        let end_level = levels[result
          .iter_backwards_from(sequence_end, runs_len - 1)
          .find(|&i| !original_classes[i].removed_by_x9())
          .unwrap_or(sequence_end - 1)];

        let preceeding_level = match original_classes[..sequence_start]
          .iter()
          .rposition(|x| !x.removed_by_x9())
        {
          Some(idx) => levels[idx],
          None => self.level,
        };

        let last_non_removed = original_classes[..sequence_end]
          .iter()
          .copied()
          .rev()
          .find(|x| !x.removed_by_x9())
          .unwrap_or(Class::BN);

        let succeeding_level = if last_non_removed.is_isolate() {
          self.level
        } else {
          match original_classes[sequence_end..]
            .iter()
            .position(|x| !x.removed_by_x9())
          {
            Some(idx) => levels[sequence_end + idx],
            None => self.level,
          }
        };

        result.start_class = max(sequence_level, preceeding_level).class();
        result.end_class = max(end_level, succeeding_level).class();

        irs.push(result);
      }
    } else {
      irs.reserve_exact(level_runs.len());

      for run in level_runs {
        let run_levels = &levels[run.clone()];
        let run_classes = &original_classes[run.clone()];
        let seq_level = run_levels[run_classes
          .iter()
          .position(|c| !c.removed_by_x9())
          .unwrap_or(0)];

        let end_level = run_levels[run_classes
          .iter()
          .rposition(|c| !c.removed_by_x9())
          .unwrap_or(run.end - run.start - 1)];

        let pred_level = match original_classes[..run.start]
          .iter()
          .rposition(|c| !c.removed_by_x9())
        {
          Some(idx) => levels[idx],
          None => self.level,
        };

        let succ_level = match original_classes[run.end..]
          .iter()
          .position(|c| !c.removed_by_x9())
        {
          Some(idx) => levels[run.end + idx],
          None => self.level,
        };

        irs.push(IsolatingRunSequence {
          runs: vec![run.clone()],
          start_class: max(seq_level, pred_level).class(),
          end_class: max(end_level, succ_level).class(),
        });
      }
    }

    Ok(())
  }
}
