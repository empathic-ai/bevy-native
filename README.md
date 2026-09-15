# 🏠 Bevy Native

**⚠️ Still in early development. ⚠️**

Use Bevy to render native user interfaces.

Platform | Support |
--- | --- | 
Web | Supported ✔️ |
Android | Planned ➡️ |
iOS | Planned ➡️ |
macOS | Pending |
Windows | Pending |
Linux | Pending |

## Web layout updates

The base renderer owns CSS display and visibility: HList/VList containers stay
flex containers on every refresh, labels use block, and ordinary controls use
grid. List updates own direction, alignment, and spacing; they must not compete
with base rendering for display. Their change-detection ticks can differ when
input values, hover transforms, or content change. ResizeObserver measurements
update Control only when dimensions differ, avoiding redundant render feedback.
Host regression tests for this contract live in src/layout.rs.
