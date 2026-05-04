use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use wit_parser::{
    Function, FunctionKind, Interface, Resolve, Type, TypeDefKind, TypeId, WorldItem,
};

fn main() -> Result<()> {
    let root = workspace_root();
    let wit_dir = root.join("wit/layer36/phase2");
    let output = root.join("docs/book/src/reference/uapi/index.md");

    let mut resolve = Resolve::default();
    let (app_package, _) = resolve
        .push_dir(&wit_dir)
        .with_context(|| format!("parse WIT package at {}", wit_dir.display()))?;

    let world_id = resolve
        .select_world(&[app_package], Some("cli"))
        .context("select layer36:app/cli world")?;

    let markdown = render_reference(&resolve, world_id)?;

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create output directory {}", parent.display()))?;
    }
    fs::write(&output, markdown).with_context(|| format!("write {}", output.display()))?;

    Ok(())
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn render_reference(resolve: &Resolve, world_id: wit_parser::WorldId) -> Result<String> {
    let world = &resolve.worlds[world_id];
    let package = world
        .package
        .map(|id| resolve.packages[id].name.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let mut out = String::new();
    writeln!(out, "# UAPI Reference")?;
    writeln!(out)?;
    writeln!(
        out,
        "> Generated from `wit/layer36/phase2`. Do not edit this page by hand."
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "Layer36 Phase 2 exposes the `{}` world from `{}`.",
        world.name, package
    )?;
    writeln!(out)?;
    writeln!(out, "The current world imports these interfaces:")?;
    writeln!(out)?;

    for (key, item) in &world.imports {
        if let WorldItem::Interface { id, .. } = item {
            let interface = &resolve.interfaces[*id];
            let name = interface_name(resolve, interface);
            writeln!(out, "- `{name}`")?;
        } else {
            writeln!(out, "- `{}`", String::from(key.clone()))?;
        }
    }

    writeln!(out)?;
    writeln!(out, "The app exports:")?;
    writeln!(out)?;
    for (key, item) in &world.exports {
        match item {
            WorldItem::Function(function) => {
                writeln!(out, "- `{}`", render_function(resolve, function))?;
            }
            _ => writeln!(out, "- `{}`", String::from(key.clone()))?,
        }
    }

    let mut interfaces = world
        .imports
        .values()
        .filter_map(|item| match item {
            WorldItem::Interface { id, .. } => Some(*id),
            _ => None,
        })
        .collect::<Vec<_>>();
    interfaces.sort_by_key(|id| interface_name(resolve, &resolve.interfaces[*id]));

    for id in interfaces {
        render_interface(resolve, id, &mut out)?;
    }

    Ok(out)
}

fn render_interface(
    resolve: &Resolve,
    id: wit_parser::InterfaceId,
    out: &mut String,
) -> Result<()> {
    let interface = &resolve.interfaces[id];
    let name = interface_name(resolve, interface);

    writeln!(out)?;
    writeln!(out, "## `{name}`")?;
    writeln!(out)?;

    let resource_type_ids = interface
        .types
        .values()
        .copied()
        .filter(|type_id| matches!(resolve.types[*type_id].kind, TypeDefKind::Resource))
        .collect::<Vec<_>>();

    let freestanding = interface
        .functions
        .values()
        .filter(|func| func.kind.resource().is_none())
        .collect::<Vec<_>>();

    if !freestanding.is_empty() {
        writeln!(out, "### Functions")?;
        writeln!(out)?;
        for func in freestanding {
            writeln!(out, "- `{}`", render_function(resolve, func))?;
        }
        writeln!(out)?;
    }

    let visible_types = interface
        .types
        .values()
        .copied()
        .filter(|type_id| is_renderable_type_def(resolve, *type_id))
        .collect::<Vec<_>>();

    if !visible_types.is_empty() {
        writeln!(out, "### Types")?;
        writeln!(out)?;
        for type_id in visible_types {
            render_type_def(resolve, type_id, out)?;
        }
    }

    for type_id in resource_type_ids {
        let methods = interface
            .functions
            .values()
            .filter(|func| func.kind.resource() == Some(type_id))
            .collect::<Vec<_>>();
        if methods.is_empty() {
            continue;
        }

        let name = resolve.types[type_id].name.as_deref().unwrap_or("resource");
        writeln!(out, "#### `{name}` methods")?;
        writeln!(out)?;
        for method in methods {
            writeln!(out, "- `{}`", render_function(resolve, method))?;
        }
        writeln!(out)?;
    }

    Ok(())
}

fn render_type_def(resolve: &Resolve, type_id: TypeId, out: &mut String) -> Result<()> {
    let ty = &resolve.types[type_id];
    let name = ty.name.as_deref().unwrap_or("anonymous");

    match &ty.kind {
        TypeDefKind::Record(record) => {
            writeln!(out, "#### `{name}` record")?;
            writeln!(out)?;
            for field in &record.fields {
                writeln!(
                    out,
                    "- `{}`: `{}`",
                    field.name,
                    render_type(resolve, &field.ty)
                )?;
            }
            writeln!(out)?;
        }
        TypeDefKind::Enum(enum_) => {
            writeln!(out, "#### `{name}` enum")?;
            writeln!(out)?;
            for case in &enum_.cases {
                writeln!(out, "- `{}`", case.name)?;
            }
            writeln!(out)?;
        }
        TypeDefKind::Variant(variant) => {
            writeln!(out, "#### `{name}` variant")?;
            writeln!(out)?;
            for case in &variant.cases {
                if let Some(ty) = &case.ty {
                    writeln!(out, "- `{}`: `{}`", case.name, render_type(resolve, ty))?;
                } else {
                    writeln!(out, "- `{}`", case.name)?;
                }
            }
            writeln!(out)?;
        }
        TypeDefKind::Resource => {
            writeln!(out, "#### `{name}` resource")?;
            writeln!(out)?;
        }
        TypeDefKind::Type(inner) => {
            writeln!(out, "#### `{name}` type")?;
            writeln!(out)?;
            writeln!(out, "`{}`", render_type(resolve, inner))?;
            writeln!(out)?;
        }
        _ => {
            writeln!(out, "#### `{name}` {}", ty.kind.as_str())?;
            writeln!(out)?;
        }
    }

    Ok(())
}

fn render_function(resolve: &Resolve, function: &Function) -> String {
    let mut rendered = String::new();
    rendered.push_str(&render_function_name(resolve, function));
    rendered.push('(');

    let params = function
        .params
        .iter()
        .filter(|(name, _)| !(function.kind.resource().is_some() && name == "self"))
        .collect::<Vec<_>>();

    for (index, (name, ty)) in params.iter().enumerate() {
        if index > 0 {
            rendered.push_str(", ");
        }
        rendered.push_str(name);
        rendered.push_str(": ");
        rendered.push_str(&render_type(resolve, ty));
    }

    rendered.push(')');

    if let Some(result) = &function.result {
        rendered.push_str(" -> ");
        rendered.push_str(&render_type(resolve, result));
    }

    if matches!(
        function.kind,
        FunctionKind::Constructor(_) | FunctionKind::Static(_) | FunctionKind::AsyncStatic(_)
    ) {
        rendered.push_str(" [resource]");
    }

    rendered
}

fn render_function_name(resolve: &Resolve, function: &Function) -> String {
    if let Some(resource_id) = function.kind.resource() {
        let resource_name = resolve.types[resource_id]
            .name
            .as_deref()
            .unwrap_or("resource");
        for prefix in [
            format!("[method]{resource_name}."),
            format!("[static]{resource_name}."),
        ] {
            if let Some(name) = function.name.strip_prefix(&prefix) {
                return name.to_string();
            }
        }
        if let Some(name) = function.name.strip_prefix("[constructor]") {
            return name.to_string();
        }
    }

    function.name.clone()
}

fn is_renderable_type_def(resolve: &Resolve, type_id: TypeId) -> bool {
    let ty = &resolve.types[type_id];
    match &ty.kind {
        TypeDefKind::Type(inner) => {
            let Some(name) = &ty.name else {
                return false;
            };
            render_type(resolve, inner) != *name
        }
        _ => true,
    }
}

fn render_type(resolve: &Resolve, ty: &Type) -> String {
    match ty {
        Type::Bool => "bool".to_string(),
        Type::U8 => "u8".to_string(),
        Type::U16 => "u16".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U64 => "u64".to_string(),
        Type::S8 => "s8".to_string(),
        Type::S16 => "s16".to_string(),
        Type::S32 => "s32".to_string(),
        Type::S64 => "s64".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Char => "char".to_string(),
        Type::String => "string".to_string(),
        Type::ErrorContext => "error-context".to_string(),
        Type::Id(id) => render_type_id(resolve, *id),
    }
}

fn render_type_id(resolve: &Resolve, id: TypeId) -> String {
    let ty = &resolve.types[id];
    if let Some(name) = &ty.name {
        return name.clone();
    }

    match &ty.kind {
        TypeDefKind::List(inner) => format!("list<{}>", render_type(resolve, inner)),
        TypeDefKind::Option(inner) => format!("option<{}>", render_type(resolve, inner)),
        TypeDefKind::Result(result) => {
            let ok = result
                .ok
                .as_ref()
                .map(|ty| render_type(resolve, ty))
                .unwrap_or_else(|| "_".to_string());
            let err = result
                .err
                .as_ref()
                .map(|ty| render_type(resolve, ty))
                .unwrap_or_else(|| "_".to_string());
            format!("result<{ok}, {err}>")
        }
        TypeDefKind::Tuple(tuple) => {
            let items = tuple
                .types
                .iter()
                .map(|ty| render_type(resolve, ty))
                .collect::<Vec<_>>()
                .join(", ");
            format!("tuple<{items}>")
        }
        TypeDefKind::Handle(handle) => match handle {
            wit_parser::Handle::Own(id) => format!("own<{}>", render_type_id(resolve, *id)),
            wit_parser::Handle::Borrow(id) => format!("borrow<{}>", render_type_id(resolve, *id)),
        },
        other => other.as_str().to_string(),
    }
}

fn interface_name(resolve: &Resolve, interface: &Interface) -> String {
    match (interface.package, interface.name.as_deref()) {
        (Some(package_id), Some(name)) => resolve.packages[package_id].name.interface_id(name),
        (_, Some(name)) => name.to_string(),
        _ => "anonymous-interface".to_string(),
    }
}
