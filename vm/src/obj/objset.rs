/*
 * Builtin set type with a sequence of unique items.
 */

use std::cell::{Cell, RefCell};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::fmt;
use std::hash::{Hash, Hasher};

use super::objbool;
use super::objint;
use super::objiter;
use super::objstr;
use super::objtype;
use crate::pyobject::{
    PyContext, PyFuncArgs, PyIteratorValue, PyObject, PyObjectRef, PyResult, PyValue, TypeProtocol,
};
use crate::vm::{ReprGuard, VirtualMachine};

#[derive(Default)]
pub struct PySet {
    elements: RefCell<HashMap<u64, PyObjectRef>>,
}

impl fmt::Debug for PySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement more detailed, non-recursive Debug formatter
        f.write_str("set")
    }
}

impl PyValue for PySet {
    fn required_type(ctx: &PyContext) -> PyObjectRef {
        ctx.set_type()
    }
}

pub fn get_elements(obj: &PyObjectRef) -> HashMap<u64, PyObjectRef> {
    obj.payload::<PySet>().unwrap().elements.borrow().clone()
}

fn perform_action_with_hash(
    vm: &mut VirtualMachine,
    elements: &mut HashMap<u64, PyObjectRef>,
    item: &PyObjectRef,
    f: &Fn(&mut VirtualMachine, &mut HashMap<u64, PyObjectRef>, u64, &PyObjectRef) -> PyResult,
) -> PyResult {
    let hash: PyObjectRef = vm.call_method(item, "__hash__", vec![])?;

    let hash_value = objint::get_value(&hash);
    let mut hasher = DefaultHasher::new();
    hash_value.hash(&mut hasher);
    let key = hasher.finish();
    f(vm, elements, key, item)
}

fn insert_into_set(
    vm: &mut VirtualMachine,
    elements: &mut HashMap<u64, PyObjectRef>,
    item: &PyObjectRef,
) -> PyResult {
    fn insert(
        vm: &mut VirtualMachine,
        elements: &mut HashMap<u64, PyObjectRef>,
        key: u64,
        value: &PyObjectRef,
    ) -> PyResult {
        elements.insert(key, value.clone());
        Ok(vm.get_none())
    }
    perform_action_with_hash(vm, elements, item, &insert)
}

fn set_add(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("set.add called with: {:?}", args);
    arg_check!(
        vm,
        args,
        required = [(zelf, Some(vm.ctx.set_type())), (item, None)]
    );
    match zelf.payload::<PySet>() {
        Some(set) => insert_into_set(vm, &mut set.elements.borrow_mut(), item),
        _ => Err(vm.new_type_error("set.add is called with no item".to_string())),
    }
}

fn set_remove(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("set.remove called with: {:?}", args);
    arg_check!(
        vm,
        args,
        required = [(s, Some(vm.ctx.set_type())), (item, None)]
    );
    match s.payload::<PySet>() {
        Some(set) => {
            fn remove(
                vm: &mut VirtualMachine,
                elements: &mut HashMap<u64, PyObjectRef>,
                key: u64,
                value: &PyObjectRef,
            ) -> PyResult {
                match elements.remove(&key) {
                    None => {
                        let item_str = format!("{:?}", value);
                        Err(vm.new_key_error(item_str))
                    }
                    Some(_) => Ok(vm.get_none()),
                }
            }
            perform_action_with_hash(vm, &mut set.elements.borrow_mut(), item, &remove)
        }
        _ => Err(vm.new_type_error("set.remove is called with no item".to_string())),
    }
}

fn set_discard(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("set.discard called with: {:?}", args);
    arg_check!(
        vm,
        args,
        required = [(s, Some(vm.ctx.set_type())), (item, None)]
    );
    match s.payload::<PySet>() {
        Some(set) => {
            fn discard(
                vm: &mut VirtualMachine,
                elements: &mut HashMap<u64, PyObjectRef>,
                key: u64,
                _value: &PyObjectRef,
            ) -> PyResult {
                elements.remove(&key);
                Ok(vm.get_none())
            }
            perform_action_with_hash(vm, &mut set.elements.borrow_mut(), item, &discard)
        }
        None => Err(vm.new_type_error("set.discard is called with no item".to_string())),
    }
}

fn set_clear(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("set.clear called");
    arg_check!(vm, args, required = [(s, Some(vm.ctx.set_type()))]);
    match s.payload::<PySet>() {
        Some(set) => {
            set.elements.borrow_mut().clear();
            Ok(vm.get_none())
        }
        None => Err(vm.new_type_error("".to_string())),
    }
}

/* Create a new object of sub-type of set */
fn set_new(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(cls, None)],
        optional = [(iterable, None)]
    );

    if !objtype::issubclass(cls, &vm.ctx.set_type()) {
        return Err(vm.new_type_error(format!("{} is not a subtype of set", cls)));
    }

    let elements: HashMap<u64, PyObjectRef> = match iterable {
        None => HashMap::new(),
        Some(iterable) => {
            let mut elements = HashMap::new();
            let iterator = objiter::get_iter(vm, iterable)?;
            while let Ok(v) = vm.call_method(&iterator, "__next__", vec![]) {
                insert_into_set(vm, &mut elements, &v)?;
            }
            elements
        }
    };

    Ok(PyObject::new(
        PySet {
            elements: RefCell::new(elements),
        },
        cls.clone(),
    ))
}

fn set_len(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("set.len called with: {:?}", args);
    arg_check!(vm, args, required = [(s, Some(vm.ctx.set_type()))]);
    let elements = get_elements(s);
    Ok(vm.context().new_int(elements.len()))
}

fn set_copy(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("set.copy called with: {:?}", args);
    arg_check!(vm, args, required = [(s, Some(vm.ctx.set_type()))]);
    let elements = get_elements(s);
    Ok(PyObject::new(
        PySet {
            elements: RefCell::new(elements),
        },
        vm.ctx.set_type(),
    ))
}

fn set_repr(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(o, Some(vm.ctx.set_type()))]);

    let elements = get_elements(o);
    let s = if elements.is_empty() {
        "set()".to_string()
    } else if let Some(_guard) = ReprGuard::enter(o) {
        let mut str_parts = vec![];
        for elem in elements.values() {
            let part = vm.to_repr(elem)?;
            str_parts.push(objstr::get_value(&part));
        }

        format!("{{{}}}", str_parts.join(", "))
    } else {
        "set(...)".to_string()
    };
    Ok(vm.new_str(s))
}

pub fn set_contains(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(set, Some(vm.ctx.set_type())), (needle, None)]
    );
    for element in get_elements(set).iter() {
        match vm._eq(needle.clone(), element.1.clone()) {
            Ok(value) => {
                if objbool::get_value(&value) {
                    return Ok(vm.new_bool(true));
                }
            }
            Err(_) => return Err(vm.new_type_error("".to_string())),
        }
    }

    Ok(vm.new_bool(false))
}

fn set_eq(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_compare_inner(
        vm,
        args,
        &|zelf: usize, other: usize| -> bool { zelf != other },
        false,
    )
}

fn set_ge(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_compare_inner(
        vm,
        args,
        &|zelf: usize, other: usize| -> bool { zelf < other },
        false,
    )
}

fn set_gt(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_compare_inner(
        vm,
        args,
        &|zelf: usize, other: usize| -> bool { zelf <= other },
        false,
    )
}

fn set_le(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_compare_inner(
        vm,
        args,
        &|zelf: usize, other: usize| -> bool { zelf < other },
        true,
    )
}

fn set_lt(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_compare_inner(
        vm,
        args,
        &|zelf: usize, other: usize| -> bool { zelf <= other },
        true,
    )
}

fn set_compare_inner(
    vm: &mut VirtualMachine,
    args: PyFuncArgs,
    size_func: &Fn(usize, usize) -> bool,
    swap: bool,
) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (zelf, Some(vm.ctx.set_type())),
            (other, Some(vm.ctx.set_type()))
        ]
    );

    let get_zelf = |swap: bool| -> &PyObjectRef {
        if swap {
            other
        } else {
            zelf
        }
    };
    let get_other = |swap: bool| -> &PyObjectRef {
        if swap {
            zelf
        } else {
            other
        }
    };

    let zelf_elements = get_elements(get_zelf(swap));
    let other_elements = get_elements(get_other(swap));
    if size_func(zelf_elements.len(), other_elements.len()) {
        return Ok(vm.new_bool(false));
    }
    for element in other_elements.iter() {
        match vm.call_method(get_zelf(swap), "__contains__", vec![element.1.clone()]) {
            Ok(value) => {
                if !objbool::get_value(&value) {
                    return Ok(vm.new_bool(false));
                }
            }
            Err(_) => return Err(vm.new_type_error("".to_string())),
        }
    }
    Ok(vm.new_bool(true))
}

fn set_union(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (zelf, Some(vm.ctx.set_type())),
            (other, Some(vm.ctx.set_type()))
        ]
    );

    let mut elements = get_elements(zelf).clone();
    elements.extend(get_elements(other).clone());

    Ok(PyObject::new(
        PySet {
            elements: RefCell::new(elements),
        },
        vm.ctx.set_type(),
    ))
}

fn set_intersection(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_combine_inner(vm, args, SetCombineOperation::Intersection)
}

fn set_difference(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_combine_inner(vm, args, SetCombineOperation::Difference)
}

fn set_symmetric_difference(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (zelf, Some(vm.ctx.set_type())),
            (other, Some(vm.ctx.set_type()))
        ]
    );

    let mut elements = HashMap::new();

    for element in get_elements(zelf).iter() {
        let value = vm.call_method(other, "__contains__", vec![element.1.clone()])?;
        if !objbool::get_value(&value) {
            elements.insert(element.0.clone(), element.1.clone());
        }
    }

    for element in get_elements(other).iter() {
        let value = vm.call_method(zelf, "__contains__", vec![element.1.clone()])?;
        if !objbool::get_value(&value) {
            elements.insert(element.0.clone(), element.1.clone());
        }
    }

    Ok(PyObject::new(
        PySet {
            elements: RefCell::new(elements),
        },
        vm.ctx.set_type(),
    ))
}

enum SetCombineOperation {
    Intersection,
    Difference,
}

fn set_combine_inner(
    vm: &mut VirtualMachine,
    args: PyFuncArgs,
    op: SetCombineOperation,
) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (zelf, Some(vm.ctx.set_type())),
            (other, Some(vm.ctx.set_type()))
        ]
    );

    let mut elements = HashMap::new();

    for element in get_elements(zelf).iter() {
        let value = vm.call_method(other, "__contains__", vec![element.1.clone()])?;
        let should_add = match op {
            SetCombineOperation::Intersection => objbool::get_value(&value),
            SetCombineOperation::Difference => !objbool::get_value(&value),
        };
        if should_add {
            elements.insert(element.0.clone(), element.1.clone());
        }
    }

    Ok(PyObject::new(
        PySet {
            elements: RefCell::new(elements),
        },
        vm.ctx.set_type(),
    ))
}

fn set_pop(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(s, Some(vm.ctx.set_type()))]);

    match s.payload::<PySet>() {
        Some(set) => {
            let mut elements = set.elements.borrow_mut();
            match elements.clone().keys().next() {
                Some(key) => Ok(elements.remove(key).unwrap()),
                None => Err(vm.new_key_error("pop from an empty set".to_string())),
            }
        }
        _ => Err(vm.new_type_error("".to_string())),
    }
}

fn set_update(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_ior(vm, args)?;
    Ok(vm.get_none())
}

fn set_ior(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(zelf, Some(vm.ctx.set_type())), (iterable, None)]
    );

    match zelf.payload::<PySet>() {
        Some(set) => {
            let iterator = objiter::get_iter(vm, iterable)?;
            while let Ok(v) = vm.call_method(&iterator, "__next__", vec![]) {
                insert_into_set(vm, &mut set.elements.borrow_mut(), &v)?;
            }
        }
        _ => return Err(vm.new_type_error("set.update is called with no other".to_string())),
    }
    Ok(zelf.clone())
}

fn set_intersection_update(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_combine_update_inner(vm, args, SetCombineOperation::Intersection)?;
    Ok(vm.get_none())
}

fn set_iand(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_combine_update_inner(vm, args, SetCombineOperation::Intersection)
}

fn set_difference_update(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_combine_update_inner(vm, args, SetCombineOperation::Difference)?;
    Ok(vm.get_none())
}

fn set_isub(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_combine_update_inner(vm, args, SetCombineOperation::Difference)
}

fn set_combine_update_inner(
    vm: &mut VirtualMachine,
    args: PyFuncArgs,
    op: SetCombineOperation,
) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(zelf, Some(vm.ctx.set_type())), (iterable, None)]
    );

    match zelf.payload::<PySet>() {
        Some(set) => {
            let mut elements = set.elements.borrow_mut();
            for element in elements.clone().iter() {
                let value = vm.call_method(iterable, "__contains__", vec![element.1.clone()])?;
                let should_remove = match op {
                    SetCombineOperation::Intersection => !objbool::get_value(&value),
                    SetCombineOperation::Difference => objbool::get_value(&value),
                };
                if should_remove {
                    elements.remove(&element.0.clone());
                }
            }
        }
        _ => return Err(vm.new_type_error("".to_string())),
    }
    Ok(zelf.clone())
}

fn set_symmetric_difference_update(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    set_ixor(vm, args)?;
    Ok(vm.get_none())
}

fn set_ixor(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(zelf, Some(vm.ctx.set_type())), (iterable, None)]
    );

    match zelf.payload::<PySet>() {
        Some(set) => {
            let elements_original = set.elements.borrow().clone();
            let iterator = objiter::get_iter(vm, iterable)?;
            while let Ok(v) = vm.call_method(&iterator, "__next__", vec![]) {
                insert_into_set(vm, &mut set.elements.borrow_mut(), &v)?;
            }
            for element in elements_original.iter() {
                let value = vm.call_method(iterable, "__contains__", vec![element.1.clone()])?;
                if objbool::get_value(&value) {
                    set.elements.borrow_mut().remove(&element.0.clone());
                }
            }
        }
        _ => return Err(vm.new_type_error("".to_string())),
    }

    Ok(zelf.clone())
}

fn set_iter(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(zelf, Some(vm.ctx.set_type()))]);

    let items = get_elements(zelf).values().cloned().collect();
    let set_list = vm.ctx.new_list(items);
    let iter_obj = PyObject::new(
        PyIteratorValue {
            position: Cell::new(0),
            iterated_obj: set_list,
        },
        vm.ctx.iter_type(),
    );

    Ok(iter_obj)
}

fn frozenset_repr(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(o, Some(vm.ctx.frozenset_type()))]);

    let elements = get_elements(o);
    let s = if elements.is_empty() {
        "frozenset()".to_string()
    } else {
        let mut str_parts = vec![];
        for elem in elements.values() {
            let part = vm.to_repr(elem)?;
            str_parts.push(objstr::get_value(&part));
        }

        format!("frozenset({{{}}})", str_parts.join(", "))
    };
    Ok(vm.new_str(s))
}

pub fn init(context: &PyContext) {
    let set_type = &context.set_type;

    let set_doc = "set() -> new empty set object\n\
                   set(iterable) -> new set object\n\n\
                   Build an unordered collection of unique elements.";

    context.set_attr(
        &set_type,
        "__contains__",
        context.new_rustfunc(set_contains),
    );
    context.set_attr(&set_type, "__len__", context.new_rustfunc(set_len));
    context.set_attr(&set_type, "__new__", context.new_rustfunc(set_new));
    context.set_attr(&set_type, "__repr__", context.new_rustfunc(set_repr));
    context.set_attr(&set_type, "__eq__", context.new_rustfunc(set_eq));
    context.set_attr(&set_type, "__ge__", context.new_rustfunc(set_ge));
    context.set_attr(&set_type, "__gt__", context.new_rustfunc(set_gt));
    context.set_attr(&set_type, "__le__", context.new_rustfunc(set_le));
    context.set_attr(&set_type, "__lt__", context.new_rustfunc(set_lt));
    context.set_attr(&set_type, "issubset", context.new_rustfunc(set_le));
    context.set_attr(&set_type, "issuperset", context.new_rustfunc(set_ge));
    context.set_attr(&set_type, "union", context.new_rustfunc(set_union));
    context.set_attr(&set_type, "__or__", context.new_rustfunc(set_union));
    context.set_attr(
        &set_type,
        "intersection",
        context.new_rustfunc(set_intersection),
    );
    context.set_attr(&set_type, "__and__", context.new_rustfunc(set_intersection));
    context.set_attr(
        &set_type,
        "difference",
        context.new_rustfunc(set_difference),
    );
    context.set_attr(&set_type, "__sub__", context.new_rustfunc(set_difference));
    context.set_attr(
        &set_type,
        "symmetric_difference",
        context.new_rustfunc(set_symmetric_difference),
    );
    context.set_attr(
        &set_type,
        "__xor__",
        context.new_rustfunc(set_symmetric_difference),
    );
    context.set_attr(&set_type, "__doc__", context.new_str(set_doc.to_string()));
    context.set_attr(&set_type, "add", context.new_rustfunc(set_add));
    context.set_attr(&set_type, "remove", context.new_rustfunc(set_remove));
    context.set_attr(&set_type, "discard", context.new_rustfunc(set_discard));
    context.set_attr(&set_type, "clear", context.new_rustfunc(set_clear));
    context.set_attr(&set_type, "copy", context.new_rustfunc(set_copy));
    context.set_attr(&set_type, "pop", context.new_rustfunc(set_pop));
    context.set_attr(&set_type, "update", context.new_rustfunc(set_update));
    context.set_attr(&set_type, "__ior__", context.new_rustfunc(set_ior));
    context.set_attr(
        &set_type,
        "intersection_update",
        context.new_rustfunc(set_intersection_update),
    );
    context.set_attr(&set_type, "__iand__", context.new_rustfunc(set_iand));
    context.set_attr(
        &set_type,
        "difference_update",
        context.new_rustfunc(set_difference_update),
    );
    context.set_attr(&set_type, "__isub__", context.new_rustfunc(set_isub));
    context.set_attr(
        &set_type,
        "symmetric_difference_update",
        context.new_rustfunc(set_symmetric_difference_update),
    );
    context.set_attr(&set_type, "__ixor__", context.new_rustfunc(set_ixor));
    context.set_attr(&set_type, "__iter__", context.new_rustfunc(set_iter));

    let frozenset_type = &context.frozenset_type;

    let frozenset_doc = "frozenset() -> empty frozenset object\n\
                         frozenset(iterable) -> frozenset object\n\n\
                         Build an immutable unordered collection of unique elements.";

    context.set_attr(
        &frozenset_type,
        "__contains__",
        context.new_rustfunc(set_contains),
    );
    context.set_attr(&frozenset_type, "__len__", context.new_rustfunc(set_len));
    context.set_attr(
        &frozenset_type,
        "__doc__",
        context.new_str(frozenset_doc.to_string()),
    );
    context.set_attr(
        &frozenset_type,
        "__repr__",
        context.new_rustfunc(frozenset_repr),
    );
}
