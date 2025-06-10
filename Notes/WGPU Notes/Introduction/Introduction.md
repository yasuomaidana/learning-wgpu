## General Model

There are several layers of abstraction between a device GPU and a web browser running the WebGPU API.
![[WGPU Diagrams.svg]]

- Physical devices have GPUs. Most devices only have one GPU, but some have more than one. Different GPU types are available:
    
    - Integrated GPUs, which live on the same board as the CPU and share its memory.
    - Discrete GPUs, which live on their own board, are separate from the CPU.
    - Software "GPUs" are implemented on the CPU.
    
    **Note:** The above diagram assumes a device with only one GPU.
    
- A native GPU API, which is part of the operating system (e.g., Metal on macOS), is a programming interface that allows native applications to utilize the GPU's capabilities. API instructions are sent to the GPU (and responses received) via a driver. A system can have multiple native OS APIs and drivers available to communicate with the GPU, although the above diagram assumes a device with only one native API/driver.
    
- A browser's WebGPU implementation handles communicating with the GPU via a native GPU API driver. A WebGPU adapter effectively represents a physical GPU and driver available on the underlying system, in your code.
    
- A logical device is an abstraction via which a single web app can access GPU capabilities in a compartmentalized way. Logical devices are required to provide multiplexing capabilities. A physical device's GPU is used by many applications and processes concurrently, including potentially many web apps. Each web app needs to be able to access WebGPU in isolation for security and logic reasons.
## WebGPU as RHI

There are several layers of abstraction between a device GPU and a web browser running the WebGPU API.

WebGPU is a _Render Hardware Interface_ (RHI), which means that it is a programming library meant to provide a **unified interface** for multiple underlying graphics hardware and operating system setups.
![[WGPU as Rhi.png]]
> The drivers do not directly provide a Render Hardware Interface (RHI) like WebGPU: we need to link to a library that implements the API on top of the low-level one that the system supports.
## Adapter and Device
The **device** is the **main object** we interact with when using WebGPU. It is the object from which we can **create** all other ones (textures, buffers, pipelines, etc.), **send instructions** to the GPU, and **handle errors**.

## The adapter 
The **adapter** is used to **access the capabilities** of the user’s hardware, which are used to select the behavior of your application among very different code paths. Once a code path is chosen, a **device** is created with **the capabilities we choose**.

Only the capabilities selected for this device are then allowed in the rest of the application. This way, it is **not possible to inadvertently rely on capabilities specific to your machine**.

![[the adapter.svg]]

> See [The Adapter - WegGPU C++ guide](https://eliemichel.github.io/LearnWebGPU/getting-started/adapter-and-device/the-adapter.html)

## The Device

A WebGPU **device** represents a **context** of use of the API. All the objects that we create (geometry, textures, etc.) are owned by the device.

The device is requested from an **adapter** by specifying the **subset of limits and features** that we are interested in. Once the device is created, the adapter should no longer be used. **The only capabilities that matter** to the application are those of the device.

## The buffer
The CPU instructs the GPU what to do by sending commands through a command queue.

