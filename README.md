# Templates CLI

Have You ever wanted to create new NextJS route which let You use query parameters using `useSearchParams` hook?  
Or insert a server side page exporting load and action functions for SvelteKit with new `satisfies` operator?  
Or make a Java Spring Controller files with all the @Annotations and routes set for GET, POST, PUT ready?

And You always needed to check the documentation, because You forget how excatly the pages looks like? Which hooks to use, what types to import, how to properly name the functions?

If yes, then <B>Templates CLI</b> is for You :).

It's a simple tool that let's You very easly create, manage and use a set of templates pages for Your projects.

Full written in <b>Rust</b> (needed to point it out, cos this is why You learn Rust, right?).

## Best way to learn is by example!

```
tp use svelte p ps e -p reports
```

This will create a `+page.svelte`, `+page.server.ts`, `+error.ts` in `./reports` folder, each containing their own template You prepared beforehand (containing types, function, whatever You want).

```
tp use next l
```

This will create a `layout.tsx` in current folder with previously prepared NextJS tempalte.

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

### Use

```
tp use <lib> [pages] -p <path>
```

- `lib` - the name of the subfolder inside Your templates folder.
- `pages` - list of shortcut inside subfolder (e.g. `p`, `ls`).
- `path` - path for where the files will be created (optional).

Using example settings from config section:

```
tp use next p lj
```

This will create a `page.ts` and `layout.js` in `./` dir and copy content of `next/[p]page.ts` and `next/[lj]layout.js` respectively inside them.

### Add

```
tp add <file> <lib> <short>
```

- `file` - the file that You want to add to templates folder
- `lib` - the name of the subfolder inside templates folder (e.g. `svelte`, `next`)
- `short` - the shortcut You want to use to access template (e.g. `p`, `ps`, `l`)

```
tp add this_page.tsx next p
```

This will copy `this_page.tsx` file into `next` subfolder inside templates folder with the `[p]` shortcut for later access (`[p]this_page.tsx')

### Print

```
tp print
```

Print the current configuration.
