//! Model helpers: diff-update a VecModel in place so Slint only repaints rows that changed.

use std::rc::Rc;

use slint::{Model, ModelRc, VecModel};

/// Replace the contents of `model` with `rows`, touching only changed indices.
pub fn sync_vec<T: Clone + PartialEq + 'static>(model: &VecModel<T>, rows: Vec<T>) {
    let old_len = model.row_count();
    let new_len = rows.len();
    for (i, row) in rows.into_iter().enumerate() {
        if i < old_len {
            if model.row_data(i).as_ref() != Some(&row) {
                model.set_row_data(i, row);
            }
        } else {
            model.push(row);
        }
    }
    for i in (new_len..old_len).rev() {
        model.remove(i);
    }
}

/// A fresh shared VecModel plus the ModelRc to hand to Slint.
pub fn new_model<T: Clone + 'static>() -> (Rc<VecModel<T>>, ModelRc<T>) {
    let m = Rc::new(VecModel::<T>::default());
    let rc = ModelRc::from(m.clone());
    (m, rc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_vec_grows_shrinks_and_updates() {
        let m = VecModel::<i32>::default();
        sync_vec(&m, vec![1, 2, 3]);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![1, 2, 3]);
        sync_vec(&m, vec![1, 5]);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![1, 5]);
        sync_vec(&m, vec![]);
        assert_eq!(m.row_count(), 0);
        sync_vec(&m, vec![7, 8, 9, 10]);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![7, 8, 9, 10]);
    }
}
