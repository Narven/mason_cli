 # mason_cli

Mason CLI allows developers to create and use reusable templates.

```bash
mason new <block_name>
```

This will create a `<block_name>` folder at the current location.

Inside the `blocks` directory, the structure will be like this:

```bash
> tree
.
└── hello
    ├── __block__
    │   └── README.md
    └── mason.yml

2 directories, 2 files    
```
