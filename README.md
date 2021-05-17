# Whoops

Whoops is a super simple cli for passing piped stdin through when the input is invalid for the program. The driving use case was parsing heterogeneous logs with jq where some lines are valid json and some are not.

Whoops loops through the stdin one line at a time and passes it to the specified program. If the program succeeds, then it returns its output. If it fails, then it returns its input.

## Usage

```
<incoming program output> | whoops <fallible program execution>
```

## Example

Simple jq with some invalid input.
```sh
cat << EOF | whoops jq 
Hello World
{ "key": "value" }
{ "invalid": json }
EOF
```

Prints:
```
Hello World
{
  "key": "value"
}
{ "invalid": json }
```

Chaining example; filtering for a specific object and formatting the rest.
```sh
# jq -e turns a null output into a non-zero exit code
cat << EOF | whoops jq -e .childJson | whoops jq 
Hello World
{ "key": "value" }
{ "invalid": json }
{ "childJson": { "childKey": "childValue" } }
EOF
```

Prints:
```
Hello World
{
  "key": "value"
}
{ "invalid": json }
{
  "childKey": "childValue"
}
```
