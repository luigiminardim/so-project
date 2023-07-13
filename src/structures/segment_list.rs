#[derive(Debug, PartialEq, Clone)]
pub struct Segment {
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, PartialEq)]
pub struct SegmentList {
    segments: Vec<Segment>,
}

impl SegmentList {
    pub fn new(segments: Vec<Segment>) -> SegmentList {
        let mut segment_list = SegmentList { segments: vec![] };
        for segment in segments {
            segment_list.free(segment);
        }
        segment_list
    }

    pub fn free(&mut self, to_free: Segment) {
        let index = self
            .segments
            .iter()
            .position(|segment| segment.offset > to_free.offset)
            .unwrap_or(self.segments.len());
        let has_to_merge_left = index > 0
            && self.segments[index - 1].offset + self.segments[index - 1].length >= to_free.offset;
        let has_to_merge_right = index < self.segments.len()
            && to_free.offset + to_free.length >= self.segments[index].offset;
        match (has_to_merge_left, has_to_merge_right) {
            (false, false) => {
                self.segments.insert(index, to_free);
            }
            (true, false) => {
                self.segments[index - 1].length =
                    to_free.offset + to_free.length - self.segments[index - 1].offset;
            }
            (false, true) => {
                self.segments[index].length =
                    self.segments[index].offset + self.segments[index].length - to_free.offset;
                self.segments[index].offset = to_free.offset;
            }
            (true, true) => {
                self.segments[index - 1].length = self.segments[index].offset
                    + self.segments[index].length
                    - self.segments[index - 1].offset;
                self.segments.remove(index);
            }
        }
    }

    pub fn alloc_segment(&mut self, to_remove: &Segment) -> Option<()> {
        let alloc_index = self.segments.iter().position(|s| {
            s.offset <= to_remove.offset
                && s.offset + s.length >= to_remove.offset + to_remove.length
        });
        match alloc_index {
            None => return None,
            Some(index) => {
                let left_remaining = Segment {
                    offset: self.segments[index].offset,
                    length: to_remove.offset - self.segments[index].offset,
                };
                let right_remaining = Segment {
                    offset: to_remove.offset + to_remove.length,
                    length: self.segments[index].offset + self.segments[index].length
                        - to_remove.offset
                        - to_remove.length,
                };
                if left_remaining.length == 0 && right_remaining.length == 0 {
                    self.segments.remove(index);
                } else if left_remaining.length == 0 {
                    self.segments[index] = right_remaining;
                } else if right_remaining.length == 0 {
                    self.segments[index] = left_remaining;
                } else {
                    self.segments[index] = left_remaining;
                    self.segments.insert(index + 1, right_remaining);
                }
                return Some(());
            }
        };
    }

    pub fn alloc(&mut self, length: usize) -> Option<Segment> {
        let original_segment = self.segments.iter().find(|s| s.length >= length)?;
        let new_segment = Segment {
            offset: original_segment.offset,
            length,
        };
        self.alloc_segment(&new_segment)?;
        Some(new_segment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn test_single_segment() {
            let segment_list = SegmentList::new(vec![Segment {
                offset: 0,
                length: 10,
            }]);
            assert_eq!(
                segment_list.segments,
                vec![Segment {
                    offset: 0,
                    length: 10
                }]
            );
        }

        #[test]
        fn test_merged_segments() {
            let segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 10,
                },
                Segment {
                    offset: 10,
                    length: 10,
                },
            ]);
            assert_eq!(
                segment_list.segments,
                vec![Segment {
                    offset: 0,
                    length: 20
                }]
            );
        }
    }

    mod free {
        use super::*;

        #[test]
        fn test_free_no_merge() {
            let mut segment_list = SegmentList::new(vec![Segment {
                offset: 0,
                length: 10,
            }]);
            segment_list.free(Segment {
                offset: 20,
                length: 10,
            });
            assert_eq!(
                segment_list.segments,
                vec![
                    Segment {
                        offset: 0,
                        length: 10
                    },
                    Segment {
                        offset: 20,
                        length: 10
                    }
                ]
            );
        }

        #[test]
        fn test_free_merge_left() {
            let mut segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 10,
                },
                Segment {
                    offset: 20,
                    length: 10,
                },
            ]);
            segment_list.free(Segment {
                offset: 10,
                length: 5,
            });
            assert_eq!(
                segment_list.segments,
                vec![
                    Segment {
                        offset: 0,
                        length: 15
                    },
                    Segment {
                        offset: 20,
                        length: 10
                    }
                ]
            );
        }

        #[test]
        fn test_free_merge_right() {
            let mut segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 10,
                },
                Segment {
                    offset: 20,
                    length: 10,
                },
            ]);
            segment_list.free(Segment {
                offset: 15,
                length: 5,
            });
            assert_eq!(
                segment_list.segments,
                vec![
                    Segment {
                        offset: 0,
                        length: 10
                    },
                    Segment {
                        offset: 15,
                        length: 15
                    }
                ]
            );
        }

        #[test]
        fn test_free_merge_both() {
            let mut segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 10,
                },
                Segment {
                    offset: 20,
                    length: 10,
                },
            ]);
            segment_list.free(Segment {
                offset: 10,
                length: 10,
            });
            assert_eq!(
                segment_list.segments,
                vec![Segment {
                    offset: 0,
                    length: 30
                }]
            );
        }
    }

    mod alloc_segment {
        use super::*;

        #[test]
        fn test_alloc_no_remaining() {
            let mut segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 5,
                },
                Segment {
                    offset: 10,
                    length: 5,
                },
                Segment {
                    offset: 30,
                    length: 5,
                },
            ]);
            segment_list.alloc_segment(&Segment {
                offset: 10,
                length: 5,
            });
            assert_eq!(
                segment_list.segments,
                vec![
                    Segment {
                        offset: 0,
                        length: 5,
                    },
                    Segment {
                        offset: 30,
                        length: 5,
                    },
                ]
            );
        }

        #[test]
        fn test_alloc_segment_with_left_remaining() {
            let mut segment_list = SegmentList::new(vec![Segment {
                offset: 0,
                length: 10,
            }]);
            segment_list.alloc_segment(&Segment {
                offset: 5,
                length: 5,
            });
            assert_eq!(
                segment_list.segments,
                vec![Segment {
                    offset: 0,
                    length: 5
                }]
            );
        }

        #[test]
        fn test_alloc_segment_with_right_remaining() {
            let mut segment_list = SegmentList::new(vec![Segment {
                offset: 0,
                length: 10,
            }]);
            segment_list.alloc_segment(&Segment {
                offset: 0,
                length: 5,
            });
            assert_eq!(
                segment_list.segments,
                vec![Segment {
                    offset: 5,
                    length: 5
                }]
            );
        }

        #[test]
        fn test_alloc_segment_with_both_remaining() {
            let mut segment_list = SegmentList::new(vec![Segment {
                offset: 0,
                length: 10,
            }]);
            segment_list.alloc_segment(&Segment {
                offset: 4,
                length: 2,
            });
            assert_eq!(
                segment_list.segments,
                vec![
                    Segment {
                        offset: 0,
                        length: 4
                    },
                    Segment {
                        offset: 6,
                        length: 4
                    }
                ]
            );
        }

        #[test]
        fn test_alloc_overflow() {
            let mut segment_list = SegmentList::new(vec![Segment {
                offset: 0,
                length: 10,
            }]);
            let result = segment_list.alloc_segment(&Segment {
                offset: 0,
                length: 20,
            });
            assert_eq!(result, None);
            assert_eq!(
                segment_list.segments,
                vec![Segment {
                    offset: 0,
                    length: 10
                }]
            );
        }

        #[test]
        fn test_alloc_no_segment() {
            let mut segment_list = SegmentList::new(vec![Segment {
                offset: 0,
                length: 10,
            }]);
            let result = segment_list.alloc_segment(&Segment {
                offset: 20,
                length: 10,
            });
            assert_eq!(result, None);
            assert_eq!(
                segment_list.segments,
                vec![Segment {
                    offset: 0,
                    length: 10
                }]
            );
        }
    }

    mod alloc {
        use super::*;

        #[test]
        fn test_alloc_first() {
            let mut segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 5,
                },
                Segment {
                    offset: 10,
                    length: 6,
                },
                Segment {
                    offset: 20,
                    length: 7,
                },
            ]);
            let result = segment_list.alloc(5);
            assert_eq!(
                result,
                Some(Segment {
                    offset: 0,
                    length: 5
                })
            );
        }

        #[test]
        fn test_alloc_middle() {
            let mut segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 5,
                },
                Segment {
                    offset: 10,
                    length: 6,
                },
                Segment {
                    offset: 20,
                    length: 7,
                },
            ]);
            let result = segment_list.alloc(6);
            assert_eq!(
                result,
                Some(Segment {
                    offset: 10,
                    length: 6
                })
            );
        }

        #[test]
        fn test_alloc_fail() {
            let mut segment_list = SegmentList::new(vec![
                Segment {
                    offset: 0,
                    length: 5,
                },
                Segment {
                    offset: 10,
                    length: 6,
                },
                Segment {
                    offset: 20,
                    length: 7,
                },
            ]);
            let result = segment_list.alloc(8);
            assert_eq!(result, None);
        }
    }
}
