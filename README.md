# Templates CLI

Have You ever wanted to create new NextJS route which let You use query parameters using `useSearchParams` hook?  
Or insert a server side page exporting load and action functions for SvelteKit with new `satisfies` operator?  
Or make a Java Spring Controller files with all the @Annotations and routes set for GET, POST, PUT ready?

And You always needed to check the documentation, because You forget how excatly the pages looks like? Which hooks to use, what types to import, how to properly name the functions?

If yes, then <B>Templates CLI</b> is for You :).

It's a simple tool that let's You very easly create, manage and use a set of templates pages for Your projects.

Full written in <b>Rust</b> (needed to point it out, cos this is why You learn Rust, right?).

## Install

```
cargo install templates
```

Please make sure that `~/.cargo/bin` is in your PATH.  
In the future more distro related options will be added :)

## Best way to learn is by example!

```
tp copy p ps -p reports -- svelte
```

This will copy two files that You have previously created in the

- ~/templates/svelte/[p]+page.svelte
- ~/templates/svelte/[ps]+page.server.ts

into:

- ./reports/+page.svelte
- ./reports/+page.server.ts

each containing their own template You prepared beforehand (containing types, function, whatever You want).

```
tp show ls
```

This will show the content of the file `~/templates/[ls]your_file_name` in the terminal, ready to be copied.

```
tp var -- next
```

This will list all the env variables that are listed in `~/templates/next/var` file.

## Usage

### Config

First thing You need to do is set up a `templates` folder. Default value is `~/templates`.

```
tp set [path_to_your_templates_folder]
```

Inside start creating Your template files with a format `[cli_shortcut]template_name`.
You can group them by subfolders (e.g. `svelte`, `next`).

Few examples:

- [r]readme.md
- [c]config.json
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
