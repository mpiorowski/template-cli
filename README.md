# Templates CLI

Have You ever wanted to insert new route which let You use page params and query using NextJS?  
Or create a server side page with load and action for SvelteKit with new `satisfies` operator?  
And You always need to check the documentation, because You always forget how excatly the pages looks like? Which hooks to use, what types to import, how to properly name the function?

If yes, then Templates cli is for You then :).

It's a simple tool that let's You fastly create, manage and use a set of templates pages.

Full written in <b>Rust</b> (needed to point it out, cos this is why You learn Rust, right?).

Best way to learn is by example!

```
tp add svelte p ps e -p reports
```

This will create a `+page.svelte`, `+page.server.ts`, `+error.ts` in `./reports` folder, each containing their own template You prepared beforehand (containing types, function, whatever You want).

## Install

```
cargo install templates
```

Please make sure that `~/.cargo/bin` is in your PATH.  
In the future more distro related options will be added :)

## Usage

### Config

First thing You need to do is set up a `templates` folder. Default value is `~/tmp`.

```
tp set [path_to_your_templates_folder]
```

Inside this folder create a subfolders for each `group` of templates (e.g. `svelte`, `next`).
Then add files inside in a format `[cli_shortcut]template_name`.

Few examples:

- `svelte/[p]+page.svelte`
- `svelte/[ls]+layout.server.ts`
- `next/[p]page.ts`
- `next/[lt]layout.ts`
- `next/[lj]layout.js`

### Add

```
tp add <lib> [pages] -p <path>
```

- `lib` - the name of the subfolder inside Yuor templates folder.
- `pages` - list of shortcut inside subfolder (e.g. `p`, `ls`).
- `path` - path for where the files will be created (optional).

Using example settings from config section:

```
tp add next p lj
```

This will create a `page.ts` and `layout.js` in `./` dir and copy content of `next/[p]page.ts` and `next/[lj]layout.js` respectively inside them.
