CS 	Code Segment
DS 	Data Segment
SS 	Stack Segment
ES 	Extra Segment
FS 	General Purpose Segments
GS


For modern use cases: 
Segmentation - CS still must point to a “code” descriptor, and DS/SS/ES/FS/GS to “data” descriptors, but all describe the same 4 GB flat space.

Why? Segmentation complicates compiler design and multitasking — paging is much more flexible and hardware-supported.

