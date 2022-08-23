# vec-string
Very short crate to display vectors Vec&lt;T> where T: Display
Normally you can do `format!("{:?}", vec);` when the elements implement `Debug`.
This crate is to do the same but for when the elements implement `Display`.
```rust
assert_eq!("[1, 2, 3]", vec![1, 2, 3].vec_string());
```

## License

```
Copyright (C) 2022 Cody Bloemhard

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
