# Propagation (`?`, `!`)

## The Basic Case

When implementing an algorithm directly from its description, the resulting code often ends up looking something like this:

```aloe
import {file} from "@std::file.aloe";
import {json} from "@std::parser/json.aloe";

let json_data = file::open("users.json");

if json_data.is_ok {
    let raw_content = json_data.read();
    let parsed_content = json::from_str(raw_content);

    if parsed_content.is_ok {
        let first_users_name = parsed_content[0].get("name");

        if first_users_name.is_ok {
            println("The name of the first user is: ${first_users_name}");
        }
    }
}
```

While correct, the code quickly becomes difficult to read due to excessive nesting. Every operation that may fail introduces another level of indentation, making the actual logic harder to follow.

A common improvement is to invert the conditions and return early whenever an error occurs.

## Removing the Nesting

```aloe
import {file} from "@std::file.aloe";
import {json} from "@std::parser/json.aloe";

fun print_the_name_of_the_first_user(file_path) {
    let json_data = file::open(file_path);
    if json_data.is_err {
        return json_data;
    }

    let raw_content = json_data.read();
    let parsed_content = json::from_str(raw_content);
    if parsed_content.is_err {
        return parsed_content;
    }

    let first_users_name = parsed_content[0].get("name");
    if first_users_name.is_err {
        return first_users_name;
    }

    println("The name of the first user is: ${first_users_name}");
}
```

This version is already much cleaner. The code now focuses on the *happy path*—the sequence of operations that should happen under normal circumstances. Error handling is still present, but it no longer dominates the control flow.

However, there is still a considerable amount of boilerplate:

```aloe
if value.is_err {
    return value;
}
```

## The Solution

Aloe provides two propagation operators:

* `?` — **error propagation**
* `!` — **panic propagation**

The `?` operator automatically returns the error if the expression evaluates to an error value.

```aloe
import {file} from "@std::file.aloe";
import {json} from "@std::parser/json.aloe";

fun print_the_name_of_the_first_user(file_path) {
    let json_data = file::open(file_path)?;

    let raw_content = json_data.read();
    let parsed_content = json::from_str(raw_content)?;

    let first_users_name = parsed_content[0].get("name")?;

    println("The name of the first user is: ${first_users_name}");
}
```

And just like that, the function becomes significantly shorter and easier to read. The control flow now clearly describes what should happen, while error propagation happens automatically in the background.

## Panic Propagation

Sometimes an error is considered unrecoverable. In such cases, continuing execution does not make sense.

The `!` operator propagates the error as a panic instead of returning it to the caller.

```aloe
import {file} from "@std::file.aloe";
import {json} from "@std::parser/json.aloe";

fun print_the_name_of_the_first_user(file_path) {
    let json_data = file::open(file_path)!; // If the file cannot be opened, abort execution.

    let raw_content = json_data.read();
    let parsed_content = json::from_str(raw_content)?;

    let first_users_name = parsed_content[0].get("name")?;

    println("The name of the first user is: ${first_users_name}");
}
```

Use `?` when the caller should have an opportunity to handle the error.

Use `!` when the error is unrecoverable and execution should stop immediately.
