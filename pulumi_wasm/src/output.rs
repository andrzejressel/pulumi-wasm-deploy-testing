use std::cell::RefCell;
use rmpv::Value;
use std::rc::Rc;

pub(crate) type OutputContentRefCell = Rc<RefCell<OutputContent>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FunctionId(pub(crate) String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FunctionSource(pub(crate) String);

static mut GLOBAL_MAP: Option<Vec<OutputContentRefCell>> = None;

pub(crate) fn access_map() -> &'static mut Vec<OutputContentRefCell> {
    let maybeMap = unsafe { &mut GLOBAL_MAP };

    match maybeMap {
        None => {
            unsafe {
                GLOBAL_MAP = Some(Vec::new());
            };
            access_map()
        }
        Some(m) => m,
    }

    // unsafe { &mut GLOBAL_MAP }
}

struct Output {
    content: OutputContent,
    tags: Vec<String>,
}

pub(crate) enum OutputContent {
    Done(Value),
    Mapped(FunctionId, FunctionSource, OutputContentRefCell),
    //TODO: Mapped multiple
    Func(OutputContentRefCell, Box<dyn Fn(Vec<u8>) -> Vec<u8>>),
    Nothing,
}

impl OutputContent {
    pub(crate) fn tpe(&self) -> &'static str {
        match self {
            OutputContent::Done(_) => "Done",
            OutputContent::Mapped(_, _, _) => "Mapped",
            OutputContent::Func(_, _) => "Func",
            OutputContent::Nothing => "Nothing",
        }
    }
}

pub(crate) fn create_nothing() -> Rc<RefCell<OutputContent>> {
    create_wrapper(OutputContent::Nothing)
}

pub(crate) fn create_new(vec: Value) -> Rc<RefCell<OutputContent>> {
    create_wrapper(OutputContent::Done(vec))
}

pub(crate) fn map_external(
    function_id: FunctionId,
    function_source: FunctionSource,
    ref_cell: OutputContentRefCell,
) -> Rc<RefCell<OutputContent>> {
    create_wrapper(OutputContent::Mapped(
        function_id,
        function_source,
        ref_cell,
    ))
}

pub(crate) fn map_internal<F>(ref_cell: OutputContentRefCell, f: F) -> Rc<RefCell<OutputContent>>
where
    F: Fn(Vec<u8>) -> Vec<u8> + 'static,
{
    create_wrapper(OutputContent::Func(ref_cell, Box::new(f)))
}

fn create_wrapper(o: OutputContent) -> Rc<RefCell<OutputContent>> {
    let rc = Rc::new(RefCell::new(o));
    access_map().push(rc.clone());
    rc
}
