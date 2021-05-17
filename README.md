# Whoops

Whoops is a super simple cli for passing piped stdin through when the input is invalid for the program. The driving use case was parsing heterogenious logs with [jq](https://stedolan.github.io/jq/) where some lines outputs are valid json and some are not.

Whoops loops through the stdin one line at a time and passes it to the specified program. If it successed, then it returns its output, if it fails, then it returns it's input.

## Usage

```
<incoming program output | whoops <fallable program execution>
```

## Example

Simple jq with some invalid input.
```sh
cat << EOF | whoops jq 
> Hello World
> { "key": "value" }
> { "invalid": json }
> EOF
```


Chaining example; filtering for a specific object and formatting the rest.
```sh
# jq -e turns a null output into a non-zero exit code
cat << EOF | whoops jq -e .childJson | whoops jq 
> Hello World
> { "key": "value" }
> { "invalid": json }
> { "child-json": { "child-key": "child-value" } }
> EOF
```
