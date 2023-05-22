#![allow(nonstandard_style)]
use super::type_alias::*;
/// Holds fn pointers for 712 GL functions
#[repr(C)]
pub struct GlFns {
  glActiveShaderProgram: Option<glActiveShaderProgram_t>,
  glActiveTexture: Option<glActiveTexture_t>,
  glAttachShader: Option<glAttachShader_t>,
  glBeginConditionalRender: Option<glBeginConditionalRender_t>,
  glBeginQuery: Option<glBeginQuery_t>,
  glBeginQueryIndexed: Option<glBeginQueryIndexed_t>,
  glBeginTransformFeedback: Option<glBeginTransformFeedback_t>,
  glBindAttribLocation: Option<glBindAttribLocation_t>,
  glBindBuffer: Option<glBindBuffer_t>,
  glBindBufferBase: Option<glBindBufferBase_t>,
  glBindBufferRange: Option<glBindBufferRange_t>,
  glBindBuffersBase: Option<glBindBuffersBase_t>,
  glBindBuffersRange: Option<glBindBuffersRange_t>,
  glBindFragDataLocation: Option<glBindFragDataLocation_t>,
  glBindFragDataLocationIndexed: Option<glBindFragDataLocationIndexed_t>,
  glBindFramebuffer: Option<glBindFramebuffer_t>,
  glBindImageTexture: Option<glBindImageTexture_t>,
  glBindImageTextures: Option<glBindImageTextures_t>,
  glBindProgramPipeline: Option<glBindProgramPipeline_t>,
  glBindRenderbuffer: Option<glBindRenderbuffer_t>,
  glBindSampler: Option<glBindSampler_t>,
  glBindSamplers: Option<glBindSamplers_t>,
  glBindTexture: Option<glBindTexture_t>,
  glBindTextureUnit: Option<glBindTextureUnit_t>,
  glBindTextures: Option<glBindTextures_t>,
  glBindTransformFeedback: Option<glBindTransformFeedback_t>,
  glBindVertexArray: Option<glBindVertexArray_t>,
  glBindVertexBuffer: Option<glBindVertexBuffer_t>,
  glBindVertexBuffers: Option<glBindVertexBuffers_t>,
  glBlendBarrier: Option<glBlendBarrier_t>,
  glBlendColor: Option<glBlendColor_t>,
  glBlendEquation: Option<glBlendEquation_t>,
  glBlendEquationSeparate: Option<glBlendEquationSeparate_t>,
  glBlendEquationSeparatei: Option<glBlendEquationSeparatei_t>,
  glBlendEquationi: Option<glBlendEquationi_t>,
  glBlendFunc: Option<glBlendFunc_t>,
  glBlendFuncSeparate: Option<glBlendFuncSeparate_t>,
  glBlendFuncSeparatei: Option<glBlendFuncSeparatei_t>,
  glBlendFunci: Option<glBlendFunci_t>,
  glBlitFramebuffer: Option<glBlitFramebuffer_t>,
  glBlitNamedFramebuffer: Option<glBlitNamedFramebuffer_t>,
  glBufferData: Option<glBufferData_t>,
  glBufferStorage: Option<glBufferStorage_t>,
  glBufferSubData: Option<glBufferSubData_t>,
  glCheckFramebufferStatus: Option<glCheckFramebufferStatus_t>,
  glCheckNamedFramebufferStatus: Option<glCheckNamedFramebufferStatus_t>,
  glClampColor: Option<glClampColor_t>,
  glClear: Option<glClear_t>,
  glClearBufferData: Option<glClearBufferData_t>,
  glClearBufferSubData: Option<glClearBufferSubData_t>,
  glClearBufferfi: Option<glClearBufferfi_t>,
  glClearBufferfv: Option<glClearBufferfv_t>,
  glClearBufferiv: Option<glClearBufferiv_t>,
  glClearBufferuiv: Option<glClearBufferuiv_t>,
  glClearColor: Option<glClearColor_t>,
  glClearDepth: Option<glClearDepth_t>,
  glClearDepthf: Option<glClearDepthf_t>,
  glClearNamedBufferData: Option<glClearNamedBufferData_t>,
  glClearNamedBufferSubData: Option<glClearNamedBufferSubData_t>,
  glClearNamedFramebufferfi: Option<glClearNamedFramebufferfi_t>,
  glClearNamedFramebufferfv: Option<glClearNamedFramebufferfv_t>,
  glClearNamedFramebufferiv: Option<glClearNamedFramebufferiv_t>,
  glClearNamedFramebufferuiv: Option<glClearNamedFramebufferuiv_t>,
  glClearStencil: Option<glClearStencil_t>,
  glClearTexImage: Option<glClearTexImage_t>,
  glClearTexSubImage: Option<glClearTexSubImage_t>,
  glClientWaitSync: Option<glClientWaitSync_t>,
  glClipControl: Option<glClipControl_t>,
  glColorMask: Option<glColorMask_t>,
  glColorMaski: Option<glColorMaski_t>,
  glColorP3ui: Option<glColorP3ui_t>,
  glColorP3uiv: Option<glColorP3uiv_t>,
  glColorP4ui: Option<glColorP4ui_t>,
  glColorP4uiv: Option<glColorP4uiv_t>,
  glCompileShader: Option<glCompileShader_t>,
  glCompressedTexImage1D: Option<glCompressedTexImage1D_t>,
  glCompressedTexImage2D: Option<glCompressedTexImage2D_t>,
  glCompressedTexImage3D: Option<glCompressedTexImage3D_t>,
  glCompressedTexSubImage1D: Option<glCompressedTexSubImage1D_t>,
  glCompressedTexSubImage2D: Option<glCompressedTexSubImage2D_t>,
  glCompressedTexSubImage3D: Option<glCompressedTexSubImage3D_t>,
  glCompressedTextureSubImage1D: Option<glCompressedTextureSubImage1D_t>,
  glCompressedTextureSubImage2D: Option<glCompressedTextureSubImage2D_t>,
  glCompressedTextureSubImage3D: Option<glCompressedTextureSubImage3D_t>,
  glCopyBufferSubData: Option<glCopyBufferSubData_t>,
  glCopyImageSubData: Option<glCopyImageSubData_t>,
  glCopyNamedBufferSubData: Option<glCopyNamedBufferSubData_t>,
  glCopyTexImage1D: Option<glCopyTexImage1D_t>,
  glCopyTexImage2D: Option<glCopyTexImage2D_t>,
  glCopyTexSubImage1D: Option<glCopyTexSubImage1D_t>,
  glCopyTexSubImage2D: Option<glCopyTexSubImage2D_t>,
  glCopyTexSubImage3D: Option<glCopyTexSubImage3D_t>,
  glCopyTextureSubImage1D: Option<glCopyTextureSubImage1D_t>,
  glCopyTextureSubImage2D: Option<glCopyTextureSubImage2D_t>,
  glCopyTextureSubImage3D: Option<glCopyTextureSubImage3D_t>,
  glCreateBuffers: Option<glCreateBuffers_t>,
  glCreateFramebuffers: Option<glCreateFramebuffers_t>,
  glCreateProgram: Option<glCreateProgram_t>,
  glCreateProgramPipelines: Option<glCreateProgramPipelines_t>,
  glCreateQueries: Option<glCreateQueries_t>,
  glCreateRenderbuffers: Option<glCreateRenderbuffers_t>,
  glCreateSamplers: Option<glCreateSamplers_t>,
  glCreateShader: Option<glCreateShader_t>,
  glCreateShaderProgramv: Option<glCreateShaderProgramv_t>,
  glCreateTextures: Option<glCreateTextures_t>,
  glCreateTransformFeedbacks: Option<glCreateTransformFeedbacks_t>,
  glCreateVertexArrays: Option<glCreateVertexArrays_t>,
  glCullFace: Option<glCullFace_t>,
  glDebugMessageCallback: Option<glDebugMessageCallback_t>,
  glDebugMessageCallbackKHR: Option<glDebugMessageCallbackKHR_t>,
  glDebugMessageControl: Option<glDebugMessageControl_t>,
  glDebugMessageControlKHR: Option<glDebugMessageControlKHR_t>,
  glDebugMessageInsert: Option<glDebugMessageInsert_t>,
  glDebugMessageInsertKHR: Option<glDebugMessageInsertKHR_t>,
  glDeleteBuffers: Option<glDeleteBuffers_t>,
  glDeleteFramebuffers: Option<glDeleteFramebuffers_t>,
  glDeleteProgram: Option<glDeleteProgram_t>,
  glDeleteProgramPipelines: Option<glDeleteProgramPipelines_t>,
  glDeleteQueries: Option<glDeleteQueries_t>,
  glDeleteRenderbuffers: Option<glDeleteRenderbuffers_t>,
  glDeleteSamplers: Option<glDeleteSamplers_t>,
  glDeleteShader: Option<glDeleteShader_t>,
  glDeleteSync: Option<glDeleteSync_t>,
  glDeleteTextures: Option<glDeleteTextures_t>,
  glDeleteTransformFeedbacks: Option<glDeleteTransformFeedbacks_t>,
  glDeleteVertexArrays: Option<glDeleteVertexArrays_t>,
  glDepthFunc: Option<glDepthFunc_t>,
  glDepthMask: Option<glDepthMask_t>,
  glDepthRange: Option<glDepthRange_t>,
  glDepthRangeArrayv: Option<glDepthRangeArrayv_t>,
  glDepthRangeIndexed: Option<glDepthRangeIndexed_t>,
  glDepthRangef: Option<glDepthRangef_t>,
  glDetachShader: Option<glDetachShader_t>,
  glDisable: Option<glDisable_t>,
  glDisableVertexArrayAttrib: Option<glDisableVertexArrayAttrib_t>,
  glDisableVertexAttribArray: Option<glDisableVertexAttribArray_t>,
  glDisablei: Option<glDisablei_t>,
  glDispatchCompute: Option<glDispatchCompute_t>,
  glDispatchComputeIndirect: Option<glDispatchComputeIndirect_t>,
  glDrawArrays: Option<glDrawArrays_t>,
  glDrawArraysIndirect: Option<glDrawArraysIndirect_t>,
  glDrawArraysInstanced: Option<glDrawArraysInstanced_t>,
  glDrawArraysInstancedBaseInstance: Option<glDrawArraysInstancedBaseInstance_t>,
  glDrawBuffer: Option<glDrawBuffer_t>,
  glDrawBuffers: Option<glDrawBuffers_t>,
  glDrawElements: Option<glDrawElements_t>,
  glDrawElementsBaseVertex: Option<glDrawElementsBaseVertex_t>,
  glDrawElementsIndirect: Option<glDrawElementsIndirect_t>,
  glDrawElementsInstanced: Option<glDrawElementsInstanced_t>,
  glDrawElementsInstancedBaseInstance: Option<glDrawElementsInstancedBaseInstance_t>,
  glDrawElementsInstancedBaseVertex: Option<glDrawElementsInstancedBaseVertex_t>,
  glDrawElementsInstancedBaseVertexBaseInstance:
    Option<glDrawElementsInstancedBaseVertexBaseInstance_t>,
  glDrawRangeElements: Option<glDrawRangeElements_t>,
  glDrawRangeElementsBaseVertex: Option<glDrawRangeElementsBaseVertex_t>,
  glDrawTransformFeedback: Option<glDrawTransformFeedback_t>,
  glDrawTransformFeedbackInstanced: Option<glDrawTransformFeedbackInstanced_t>,
  glDrawTransformFeedbackStream: Option<glDrawTransformFeedbackStream_t>,
  glDrawTransformFeedbackStreamInstanced: Option<glDrawTransformFeedbackStreamInstanced_t>,
  glEnable: Option<glEnable_t>,
  glEnableVertexArrayAttrib: Option<glEnableVertexArrayAttrib_t>,
  glEnableVertexAttribArray: Option<glEnableVertexAttribArray_t>,
  glEnablei: Option<glEnablei_t>,
  glEndConditionalRender: Option<glEndConditionalRender_t>,
  glEndQuery: Option<glEndQuery_t>,
  glEndQueryIndexed: Option<glEndQueryIndexed_t>,
  glEndTransformFeedback: Option<glEndTransformFeedback_t>,
  glFenceSync: Option<glFenceSync_t>,
  glFinish: Option<glFinish_t>,
  glFlush: Option<glFlush_t>,
  glFlushMappedBufferRange: Option<glFlushMappedBufferRange_t>,
  glFlushMappedNamedBufferRange: Option<glFlushMappedNamedBufferRange_t>,
  glFramebufferParameteri: Option<glFramebufferParameteri_t>,
  glFramebufferRenderbuffer: Option<glFramebufferRenderbuffer_t>,
  glFramebufferTexture: Option<glFramebufferTexture_t>,
  glFramebufferTexture1D: Option<glFramebufferTexture1D_t>,
  glFramebufferTexture2D: Option<glFramebufferTexture2D_t>,
  glFramebufferTexture3D: Option<glFramebufferTexture3D_t>,
  glFramebufferTextureLayer: Option<glFramebufferTextureLayer_t>,
  glFrontFace: Option<glFrontFace_t>,
  glGenBuffers: Option<glGenBuffers_t>,
  glGenFramebuffers: Option<glGenFramebuffers_t>,
  glGenProgramPipelines: Option<glGenProgramPipelines_t>,
  glGenQueries: Option<glGenQueries_t>,
  glGenRenderbuffers: Option<glGenRenderbuffers_t>,
  glGenSamplers: Option<glGenSamplers_t>,
  glGenTextures: Option<glGenTextures_t>,
  glGenTransformFeedbacks: Option<glGenTransformFeedbacks_t>,
  glGenVertexArrays: Option<glGenVertexArrays_t>,
  glGenerateMipmap: Option<glGenerateMipmap_t>,
  glGenerateTextureMipmap: Option<glGenerateTextureMipmap_t>,
  glGetActiveAtomicCounterBufferiv: Option<glGetActiveAtomicCounterBufferiv_t>,
  glGetActiveAttrib: Option<glGetActiveAttrib_t>,
  glGetActiveSubroutineName: Option<glGetActiveSubroutineName_t>,
  glGetActiveSubroutineUniformName: Option<glGetActiveSubroutineUniformName_t>,
  glGetActiveSubroutineUniformiv: Option<glGetActiveSubroutineUniformiv_t>,
  glGetActiveUniform: Option<glGetActiveUniform_t>,
  glGetActiveUniformBlockName: Option<glGetActiveUniformBlockName_t>,
  glGetActiveUniformBlockiv: Option<glGetActiveUniformBlockiv_t>,
  glGetActiveUniformName: Option<glGetActiveUniformName_t>,
  glGetActiveUniformsiv: Option<glGetActiveUniformsiv_t>,
  glGetAttachedShaders: Option<glGetAttachedShaders_t>,
  glGetAttribLocation: Option<glGetAttribLocation_t>,
  glGetBooleani_v: Option<glGetBooleani_v_t>,
  glGetBooleanv: Option<glGetBooleanv_t>,
  glGetBufferParameteri64v: Option<glGetBufferParameteri64v_t>,
  glGetBufferParameteriv: Option<glGetBufferParameteriv_t>,
  glGetBufferPointerv: Option<glGetBufferPointerv_t>,
  glGetBufferSubData: Option<glGetBufferSubData_t>,
  glGetCompressedTexImage: Option<glGetCompressedTexImage_t>,
  glGetCompressedTextureImage: Option<glGetCompressedTextureImage_t>,
  glGetCompressedTextureSubImage: Option<glGetCompressedTextureSubImage_t>,
  glGetDebugMessageLog: Option<glGetDebugMessageLog_t>,
  glGetDebugMessageLogKHR: Option<glGetDebugMessageLogKHR_t>,
  glGetDoublei_v: Option<glGetDoublei_v_t>,
  glGetDoublev: Option<glGetDoublev_t>,
  glGetError: Option<glGetError_t>,
  glGetFloati_v: Option<glGetFloati_v_t>,
  glGetFloatv: Option<glGetFloatv_t>,
  glGetFragDataIndex: Option<glGetFragDataIndex_t>,
  glGetFragDataLocation: Option<glGetFragDataLocation_t>,
  glGetFramebufferAttachmentParameteriv: Option<glGetFramebufferAttachmentParameteriv_t>,
  glGetFramebufferParameteriv: Option<glGetFramebufferParameteriv_t>,
  glGetGraphicsResetStatus: Option<glGetGraphicsResetStatus_t>,
  glGetInteger64i_v: Option<glGetInteger64i_v_t>,
  glGetInteger64v: Option<glGetInteger64v_t>,
  glGetIntegeri_v: Option<glGetIntegeri_v_t>,
  glGetIntegerv: Option<glGetIntegerv_t>,
  glGetInternalformati64v: Option<glGetInternalformati64v_t>,
  glGetInternalformativ: Option<glGetInternalformativ_t>,
  glGetMultisamplefv: Option<glGetMultisamplefv_t>,
  glGetNamedBufferParameteri64v: Option<glGetNamedBufferParameteri64v_t>,
  glGetNamedBufferParameteriv: Option<glGetNamedBufferParameteriv_t>,
  glGetNamedBufferPointerv: Option<glGetNamedBufferPointerv_t>,
  glGetNamedBufferSubData: Option<glGetNamedBufferSubData_t>,
  glGetNamedFramebufferAttachmentParameteriv: Option<glGetNamedFramebufferAttachmentParameteriv_t>,
  glGetNamedFramebufferParameteriv: Option<glGetNamedFramebufferParameteriv_t>,
  glGetNamedRenderbufferParameteriv: Option<glGetNamedRenderbufferParameteriv_t>,
  glGetObjectLabel: Option<glGetObjectLabel_t>,
  glGetObjectLabelKHR: Option<glGetObjectLabelKHR_t>,
  glGetObjectPtrLabel: Option<glGetObjectPtrLabel_t>,
  glGetObjectPtrLabelKHR: Option<glGetObjectPtrLabelKHR_t>,
  glGetPointerv: Option<glGetPointerv_t>,
  glGetPointervKHR: Option<glGetPointervKHR_t>,
  glGetProgramBinary: Option<glGetProgramBinary_t>,
  glGetProgramInfoLog: Option<glGetProgramInfoLog_t>,
  glGetProgramInterfaceiv: Option<glGetProgramInterfaceiv_t>,
  glGetProgramPipelineInfoLog: Option<glGetProgramPipelineInfoLog_t>,
  glGetProgramPipelineiv: Option<glGetProgramPipelineiv_t>,
  glGetProgramResourceIndex: Option<glGetProgramResourceIndex_t>,
  glGetProgramResourceLocation: Option<glGetProgramResourceLocation_t>,
  glGetProgramResourceLocationIndex: Option<glGetProgramResourceLocationIndex_t>,
  glGetProgramResourceName: Option<glGetProgramResourceName_t>,
  glGetProgramResourceiv: Option<glGetProgramResourceiv_t>,
  glGetProgramStageiv: Option<glGetProgramStageiv_t>,
  glGetProgramiv: Option<glGetProgramiv_t>,
  glGetQueryBufferObjecti64v: Option<glGetQueryBufferObjecti64v_t>,
  glGetQueryBufferObjectiv: Option<glGetQueryBufferObjectiv_t>,
  glGetQueryBufferObjectui64v: Option<glGetQueryBufferObjectui64v_t>,
  glGetQueryBufferObjectuiv: Option<glGetQueryBufferObjectuiv_t>,
  glGetQueryIndexediv: Option<glGetQueryIndexediv_t>,
  glGetQueryObjecti64v: Option<glGetQueryObjecti64v_t>,
  glGetQueryObjectiv: Option<glGetQueryObjectiv_t>,
  glGetQueryObjectui64v: Option<glGetQueryObjectui64v_t>,
  glGetQueryObjectuiv: Option<glGetQueryObjectuiv_t>,
  glGetQueryiv: Option<glGetQueryiv_t>,
  glGetRenderbufferParameteriv: Option<glGetRenderbufferParameteriv_t>,
  glGetSamplerParameterIiv: Option<glGetSamplerParameterIiv_t>,
  glGetSamplerParameterIuiv: Option<glGetSamplerParameterIuiv_t>,
  glGetSamplerParameterfv: Option<glGetSamplerParameterfv_t>,
  glGetSamplerParameteriv: Option<glGetSamplerParameteriv_t>,
  glGetShaderInfoLog: Option<glGetShaderInfoLog_t>,
  glGetShaderPrecisionFormat: Option<glGetShaderPrecisionFormat_t>,
  glGetShaderSource: Option<glGetShaderSource_t>,
  glGetShaderiv: Option<glGetShaderiv_t>,
  glGetString: Option<glGetString_t>,
  glGetStringi: Option<glGetStringi_t>,
  glGetSubroutineIndex: Option<glGetSubroutineIndex_t>,
  glGetSubroutineUniformLocation: Option<glGetSubroutineUniformLocation_t>,
  glGetSynciv: Option<glGetSynciv_t>,
  glGetTexImage: Option<glGetTexImage_t>,
  glGetTexLevelParameterfv: Option<glGetTexLevelParameterfv_t>,
  glGetTexLevelParameteriv: Option<glGetTexLevelParameteriv_t>,
  glGetTexParameterIiv: Option<glGetTexParameterIiv_t>,
  glGetTexParameterIuiv: Option<glGetTexParameterIuiv_t>,
  glGetTexParameterfv: Option<glGetTexParameterfv_t>,
  glGetTexParameteriv: Option<glGetTexParameteriv_t>,
  glGetTextureImage: Option<glGetTextureImage_t>,
  glGetTextureLevelParameterfv: Option<glGetTextureLevelParameterfv_t>,
  glGetTextureLevelParameteriv: Option<glGetTextureLevelParameteriv_t>,
  glGetTextureParameterIiv: Option<glGetTextureParameterIiv_t>,
  glGetTextureParameterIuiv: Option<glGetTextureParameterIuiv_t>,
  glGetTextureParameterfv: Option<glGetTextureParameterfv_t>,
  glGetTextureParameteriv: Option<glGetTextureParameteriv_t>,
  glGetTextureSubImage: Option<glGetTextureSubImage_t>,
  glGetTransformFeedbackVarying: Option<glGetTransformFeedbackVarying_t>,
  glGetTransformFeedbacki64_v: Option<glGetTransformFeedbacki64_v_t>,
  glGetTransformFeedbacki_v: Option<glGetTransformFeedbacki_v_t>,
  glGetTransformFeedbackiv: Option<glGetTransformFeedbackiv_t>,
  glGetUniformBlockIndex: Option<glGetUniformBlockIndex_t>,
  glGetUniformIndices: Option<glGetUniformIndices_t>,
  glGetUniformLocation: Option<glGetUniformLocation_t>,
  glGetUniformSubroutineuiv: Option<glGetUniformSubroutineuiv_t>,
  glGetUniformdv: Option<glGetUniformdv_t>,
  glGetUniformfv: Option<glGetUniformfv_t>,
  glGetUniformiv: Option<glGetUniformiv_t>,
  glGetUniformuiv: Option<glGetUniformuiv_t>,
  glGetVertexArrayIndexed64iv: Option<glGetVertexArrayIndexed64iv_t>,
  glGetVertexArrayIndexediv: Option<glGetVertexArrayIndexediv_t>,
  glGetVertexArrayiv: Option<glGetVertexArrayiv_t>,
  glGetVertexAttribIiv: Option<glGetVertexAttribIiv_t>,
  glGetVertexAttribIuiv: Option<glGetVertexAttribIuiv_t>,
  glGetVertexAttribLdv: Option<glGetVertexAttribLdv_t>,
  glGetVertexAttribPointerv: Option<glGetVertexAttribPointerv_t>,
  glGetVertexAttribdv: Option<glGetVertexAttribdv_t>,
  glGetVertexAttribfv: Option<glGetVertexAttribfv_t>,
  glGetVertexAttribiv: Option<glGetVertexAttribiv_t>,
  glGetnColorTable: Option<glGetnColorTable_t>,
  glGetnCompressedTexImage: Option<glGetnCompressedTexImage_t>,
  glGetnConvolutionFilter: Option<glGetnConvolutionFilter_t>,
  glGetnHistogram: Option<glGetnHistogram_t>,
  glGetnMapdv: Option<glGetnMapdv_t>,
  glGetnMapfv: Option<glGetnMapfv_t>,
  glGetnMapiv: Option<glGetnMapiv_t>,
  glGetnMinmax: Option<glGetnMinmax_t>,
  glGetnPixelMapfv: Option<glGetnPixelMapfv_t>,
  glGetnPixelMapuiv: Option<glGetnPixelMapuiv_t>,
  glGetnPixelMapusv: Option<glGetnPixelMapusv_t>,
  glGetnPolygonStipple: Option<glGetnPolygonStipple_t>,
  glGetnSeparableFilter: Option<glGetnSeparableFilter_t>,
  glGetnTexImage: Option<glGetnTexImage_t>,
  glGetnUniformdv: Option<glGetnUniformdv_t>,
  glGetnUniformfv: Option<glGetnUniformfv_t>,
  glGetnUniformiv: Option<glGetnUniformiv_t>,
  glGetnUniformuiv: Option<glGetnUniformuiv_t>,
  glHint: Option<glHint_t>,
  glInvalidateBufferData: Option<glInvalidateBufferData_t>,
  glInvalidateBufferSubData: Option<glInvalidateBufferSubData_t>,
  glInvalidateFramebuffer: Option<glInvalidateFramebuffer_t>,
  glInvalidateNamedFramebufferData: Option<glInvalidateNamedFramebufferData_t>,
  glInvalidateNamedFramebufferSubData: Option<glInvalidateNamedFramebufferSubData_t>,
  glInvalidateSubFramebuffer: Option<glInvalidateSubFramebuffer_t>,
  glInvalidateTexImage: Option<glInvalidateTexImage_t>,
  glInvalidateTexSubImage: Option<glInvalidateTexSubImage_t>,
  glIsBuffer: Option<glIsBuffer_t>,
  glIsEnabled: Option<glIsEnabled_t>,
  glIsEnabledi: Option<glIsEnabledi_t>,
  glIsFramebuffer: Option<glIsFramebuffer_t>,
  glIsProgram: Option<glIsProgram_t>,
  glIsProgramPipeline: Option<glIsProgramPipeline_t>,
  glIsQuery: Option<glIsQuery_t>,
  glIsRenderbuffer: Option<glIsRenderbuffer_t>,
  glIsSampler: Option<glIsSampler_t>,
  glIsShader: Option<glIsShader_t>,
  glIsSync: Option<glIsSync_t>,
  glIsTexture: Option<glIsTexture_t>,
  glIsTransformFeedback: Option<glIsTransformFeedback_t>,
  glIsVertexArray: Option<glIsVertexArray_t>,
  glLineWidth: Option<glLineWidth_t>,
  glLinkProgram: Option<glLinkProgram_t>,
  glLogicOp: Option<glLogicOp_t>,
  glMapBuffer: Option<glMapBuffer_t>,
  glMapBufferRange: Option<glMapBufferRange_t>,
  glMapNamedBuffer: Option<glMapNamedBuffer_t>,
  glMapNamedBufferRange: Option<glMapNamedBufferRange_t>,
  glMemoryBarrier: Option<glMemoryBarrier_t>,
  glMemoryBarrierByRegion: Option<glMemoryBarrierByRegion_t>,
  glMinSampleShading: Option<glMinSampleShading_t>,
  glMultiDrawArrays: Option<glMultiDrawArrays_t>,
  glMultiDrawArraysIndirect: Option<glMultiDrawArraysIndirect_t>,
  glMultiDrawArraysIndirectCount: Option<glMultiDrawArraysIndirectCount_t>,
  glMultiDrawElements: Option<glMultiDrawElements_t>,
  glMultiDrawElementsBaseVertex: Option<glMultiDrawElementsBaseVertex_t>,
  glMultiDrawElementsIndirect: Option<glMultiDrawElementsIndirect_t>,
  glMultiDrawElementsIndirectCount: Option<glMultiDrawElementsIndirectCount_t>,
  glMultiTexCoordP1ui: Option<glMultiTexCoordP1ui_t>,
  glMultiTexCoordP1uiv: Option<glMultiTexCoordP1uiv_t>,
  glMultiTexCoordP2ui: Option<glMultiTexCoordP2ui_t>,
  glMultiTexCoordP2uiv: Option<glMultiTexCoordP2uiv_t>,
  glMultiTexCoordP3ui: Option<glMultiTexCoordP3ui_t>,
  glMultiTexCoordP3uiv: Option<glMultiTexCoordP3uiv_t>,
  glMultiTexCoordP4ui: Option<glMultiTexCoordP4ui_t>,
  glMultiTexCoordP4uiv: Option<glMultiTexCoordP4uiv_t>,
  glNamedBufferData: Option<glNamedBufferData_t>,
  glNamedBufferStorage: Option<glNamedBufferStorage_t>,
  glNamedBufferSubData: Option<glNamedBufferSubData_t>,
  glNamedFramebufferDrawBuffer: Option<glNamedFramebufferDrawBuffer_t>,
  glNamedFramebufferDrawBuffers: Option<glNamedFramebufferDrawBuffers_t>,
  glNamedFramebufferParameteri: Option<glNamedFramebufferParameteri_t>,
  glNamedFramebufferReadBuffer: Option<glNamedFramebufferReadBuffer_t>,
  glNamedFramebufferRenderbuffer: Option<glNamedFramebufferRenderbuffer_t>,
  glNamedFramebufferTexture: Option<glNamedFramebufferTexture_t>,
  glNamedFramebufferTextureLayer: Option<glNamedFramebufferTextureLayer_t>,
  glNamedRenderbufferStorage: Option<glNamedRenderbufferStorage_t>,
  glNamedRenderbufferStorageMultisample: Option<glNamedRenderbufferStorageMultisample_t>,
  glNormalP3ui: Option<glNormalP3ui_t>,
  glNormalP3uiv: Option<glNormalP3uiv_t>,
  glObjectLabel: Option<glObjectLabel_t>,
  glObjectLabelKHR: Option<glObjectLabelKHR_t>,
  glObjectPtrLabel: Option<glObjectPtrLabel_t>,
  glObjectPtrLabelKHR: Option<glObjectPtrLabelKHR_t>,
  glPatchParameterfv: Option<glPatchParameterfv_t>,
  glPatchParameteri: Option<glPatchParameteri_t>,
  glPauseTransformFeedback: Option<glPauseTransformFeedback_t>,
  glPixelStoref: Option<glPixelStoref_t>,
  glPixelStorei: Option<glPixelStorei_t>,
  glPointParameterf: Option<glPointParameterf_t>,
  glPointParameterfv: Option<glPointParameterfv_t>,
  glPointParameteri: Option<glPointParameteri_t>,
  glPointParameteriv: Option<glPointParameteriv_t>,
  glPointSize: Option<glPointSize_t>,
  glPolygonMode: Option<glPolygonMode_t>,
  glPolygonOffset: Option<glPolygonOffset_t>,
  glPolygonOffsetClamp: Option<glPolygonOffsetClamp_t>,
  glPopDebugGroup: Option<glPopDebugGroup_t>,
  glPopDebugGroupKHR: Option<glPopDebugGroupKHR_t>,
  glPrimitiveBoundingBox: Option<glPrimitiveBoundingBox_t>,
  glPrimitiveRestartIndex: Option<glPrimitiveRestartIndex_t>,
  glProgramBinary: Option<glProgramBinary_t>,
  glProgramParameteri: Option<glProgramParameteri_t>,
  glProgramUniform1d: Option<glProgramUniform1d_t>,
  glProgramUniform1dv: Option<glProgramUniform1dv_t>,
  glProgramUniform1f: Option<glProgramUniform1f_t>,
  glProgramUniform1fv: Option<glProgramUniform1fv_t>,
  glProgramUniform1i: Option<glProgramUniform1i_t>,
  glProgramUniform1iv: Option<glProgramUniform1iv_t>,
  glProgramUniform1ui: Option<glProgramUniform1ui_t>,
  glProgramUniform1uiv: Option<glProgramUniform1uiv_t>,
  glProgramUniform2d: Option<glProgramUniform2d_t>,
  glProgramUniform2dv: Option<glProgramUniform2dv_t>,
  glProgramUniform2f: Option<glProgramUniform2f_t>,
  glProgramUniform2fv: Option<glProgramUniform2fv_t>,
  glProgramUniform2i: Option<glProgramUniform2i_t>,
  glProgramUniform2iv: Option<glProgramUniform2iv_t>,
  glProgramUniform2ui: Option<glProgramUniform2ui_t>,
  glProgramUniform2uiv: Option<glProgramUniform2uiv_t>,
  glProgramUniform3d: Option<glProgramUniform3d_t>,
  glProgramUniform3dv: Option<glProgramUniform3dv_t>,
  glProgramUniform3f: Option<glProgramUniform3f_t>,
  glProgramUniform3fv: Option<glProgramUniform3fv_t>,
  glProgramUniform3i: Option<glProgramUniform3i_t>,
  glProgramUniform3iv: Option<glProgramUniform3iv_t>,
  glProgramUniform3ui: Option<glProgramUniform3ui_t>,
  glProgramUniform3uiv: Option<glProgramUniform3uiv_t>,
  glProgramUniform4d: Option<glProgramUniform4d_t>,
  glProgramUniform4dv: Option<glProgramUniform4dv_t>,
  glProgramUniform4f: Option<glProgramUniform4f_t>,
  glProgramUniform4fv: Option<glProgramUniform4fv_t>,
  glProgramUniform4i: Option<glProgramUniform4i_t>,
  glProgramUniform4iv: Option<glProgramUniform4iv_t>,
  glProgramUniform4ui: Option<glProgramUniform4ui_t>,
  glProgramUniform4uiv: Option<glProgramUniform4uiv_t>,
  glProgramUniformMatrix2dv: Option<glProgramUniformMatrix2dv_t>,
  glProgramUniformMatrix2fv: Option<glProgramUniformMatrix2fv_t>,
  glProgramUniformMatrix2x3dv: Option<glProgramUniformMatrix2x3dv_t>,
  glProgramUniformMatrix2x3fv: Option<glProgramUniformMatrix2x3fv_t>,
  glProgramUniformMatrix2x4dv: Option<glProgramUniformMatrix2x4dv_t>,
  glProgramUniformMatrix2x4fv: Option<glProgramUniformMatrix2x4fv_t>,
  glProgramUniformMatrix3dv: Option<glProgramUniformMatrix3dv_t>,
  glProgramUniformMatrix3fv: Option<glProgramUniformMatrix3fv_t>,
  glProgramUniformMatrix3x2dv: Option<glProgramUniformMatrix3x2dv_t>,
  glProgramUniformMatrix3x2fv: Option<glProgramUniformMatrix3x2fv_t>,
  glProgramUniformMatrix3x4dv: Option<glProgramUniformMatrix3x4dv_t>,
  glProgramUniformMatrix3x4fv: Option<glProgramUniformMatrix3x4fv_t>,
  glProgramUniformMatrix4dv: Option<glProgramUniformMatrix4dv_t>,
  glProgramUniformMatrix4fv: Option<glProgramUniformMatrix4fv_t>,
  glProgramUniformMatrix4x2dv: Option<glProgramUniformMatrix4x2dv_t>,
  glProgramUniformMatrix4x2fv: Option<glProgramUniformMatrix4x2fv_t>,
  glProgramUniformMatrix4x3dv: Option<glProgramUniformMatrix4x3dv_t>,
  glProgramUniformMatrix4x3fv: Option<glProgramUniformMatrix4x3fv_t>,
  glProvokingVertex: Option<glProvokingVertex_t>,
  glPushDebugGroup: Option<glPushDebugGroup_t>,
  glPushDebugGroupKHR: Option<glPushDebugGroupKHR_t>,
  glQueryCounter: Option<glQueryCounter_t>,
  glReadBuffer: Option<glReadBuffer_t>,
  glReadPixels: Option<glReadPixels_t>,
  glReadnPixels: Option<glReadnPixels_t>,
  glReleaseShaderCompiler: Option<glReleaseShaderCompiler_t>,
  glRenderbufferStorage: Option<glRenderbufferStorage_t>,
  glRenderbufferStorageMultisample: Option<glRenderbufferStorageMultisample_t>,
  glResumeTransformFeedback: Option<glResumeTransformFeedback_t>,
  glSampleCoverage: Option<glSampleCoverage_t>,
  glSampleMaski: Option<glSampleMaski_t>,
  glSamplerParameterIiv: Option<glSamplerParameterIiv_t>,
  glSamplerParameterIuiv: Option<glSamplerParameterIuiv_t>,
  glSamplerParameterf: Option<glSamplerParameterf_t>,
  glSamplerParameterfv: Option<glSamplerParameterfv_t>,
  glSamplerParameteri: Option<glSamplerParameteri_t>,
  glSamplerParameteriv: Option<glSamplerParameteriv_t>,
  glScissor: Option<glScissor_t>,
  glScissorArrayv: Option<glScissorArrayv_t>,
  glScissorIndexed: Option<glScissorIndexed_t>,
  glScissorIndexedv: Option<glScissorIndexedv_t>,
  glSecondaryColorP3ui: Option<glSecondaryColorP3ui_t>,
  glSecondaryColorP3uiv: Option<glSecondaryColorP3uiv_t>,
  glShaderBinary: Option<glShaderBinary_t>,
  glShaderSource: Option<glShaderSource_t>,
  glShaderStorageBlockBinding: Option<glShaderStorageBlockBinding_t>,
  glSpecializeShader: Option<glSpecializeShader_t>,
  glStencilFunc: Option<glStencilFunc_t>,
  glStencilFuncSeparate: Option<glStencilFuncSeparate_t>,
  glStencilMask: Option<glStencilMask_t>,
  glStencilMaskSeparate: Option<glStencilMaskSeparate_t>,
  glStencilOp: Option<glStencilOp_t>,
  glStencilOpSeparate: Option<glStencilOpSeparate_t>,
  glTexBuffer: Option<glTexBuffer_t>,
  glTexBufferRange: Option<glTexBufferRange_t>,
  glTexCoordP1ui: Option<glTexCoordP1ui_t>,
  glTexCoordP1uiv: Option<glTexCoordP1uiv_t>,
  glTexCoordP2ui: Option<glTexCoordP2ui_t>,
  glTexCoordP2uiv: Option<glTexCoordP2uiv_t>,
  glTexCoordP3ui: Option<glTexCoordP3ui_t>,
  glTexCoordP3uiv: Option<glTexCoordP3uiv_t>,
  glTexCoordP4ui: Option<glTexCoordP4ui_t>,
  glTexCoordP4uiv: Option<glTexCoordP4uiv_t>,
  glTexImage1D: Option<glTexImage1D_t>,
  glTexImage2D: Option<glTexImage2D_t>,
  glTexImage2DMultisample: Option<glTexImage2DMultisample_t>,
  glTexImage3D: Option<glTexImage3D_t>,
  glTexImage3DMultisample: Option<glTexImage3DMultisample_t>,
  glTexParameterIiv: Option<glTexParameterIiv_t>,
  glTexParameterIuiv: Option<glTexParameterIuiv_t>,
  glTexParameterf: Option<glTexParameterf_t>,
  glTexParameterfv: Option<glTexParameterfv_t>,
  glTexParameteri: Option<glTexParameteri_t>,
  glTexParameteriv: Option<glTexParameteriv_t>,
  glTexStorage1D: Option<glTexStorage1D_t>,
  glTexStorage2D: Option<glTexStorage2D_t>,
  glTexStorage2DMultisample: Option<glTexStorage2DMultisample_t>,
  glTexStorage3D: Option<glTexStorage3D_t>,
  glTexStorage3DMultisample: Option<glTexStorage3DMultisample_t>,
  glTexSubImage1D: Option<glTexSubImage1D_t>,
  glTexSubImage2D: Option<glTexSubImage2D_t>,
  glTexSubImage3D: Option<glTexSubImage3D_t>,
  glTextureBarrier: Option<glTextureBarrier_t>,
  glTextureBuffer: Option<glTextureBuffer_t>,
  glTextureBufferRange: Option<glTextureBufferRange_t>,
  glTextureParameterIiv: Option<glTextureParameterIiv_t>,
  glTextureParameterIuiv: Option<glTextureParameterIuiv_t>,
  glTextureParameterf: Option<glTextureParameterf_t>,
  glTextureParameterfv: Option<glTextureParameterfv_t>,
  glTextureParameteri: Option<glTextureParameteri_t>,
  glTextureParameteriv: Option<glTextureParameteriv_t>,
  glTextureStorage1D: Option<glTextureStorage1D_t>,
  glTextureStorage2D: Option<glTextureStorage2D_t>,
  glTextureStorage2DMultisample: Option<glTextureStorage2DMultisample_t>,
  glTextureStorage3D: Option<glTextureStorage3D_t>,
  glTextureStorage3DMultisample: Option<glTextureStorage3DMultisample_t>,
  glTextureSubImage1D: Option<glTextureSubImage1D_t>,
  glTextureSubImage2D: Option<glTextureSubImage2D_t>,
  glTextureSubImage3D: Option<glTextureSubImage3D_t>,
  glTextureView: Option<glTextureView_t>,
  glTransformFeedbackBufferBase: Option<glTransformFeedbackBufferBase_t>,
  glTransformFeedbackBufferRange: Option<glTransformFeedbackBufferRange_t>,
  glTransformFeedbackVaryings: Option<glTransformFeedbackVaryings_t>,
  glUniform1d: Option<glUniform1d_t>,
  glUniform1dv: Option<glUniform1dv_t>,
  glUniform1f: Option<glUniform1f_t>,
  glUniform1fv: Option<glUniform1fv_t>,
  glUniform1i: Option<glUniform1i_t>,
  glUniform1iv: Option<glUniform1iv_t>,
  glUniform1ui: Option<glUniform1ui_t>,
  glUniform1uiv: Option<glUniform1uiv_t>,
  glUniform2d: Option<glUniform2d_t>,
  glUniform2dv: Option<glUniform2dv_t>,
  glUniform2f: Option<glUniform2f_t>,
  glUniform2fv: Option<glUniform2fv_t>,
  glUniform2i: Option<glUniform2i_t>,
  glUniform2iv: Option<glUniform2iv_t>,
  glUniform2ui: Option<glUniform2ui_t>,
  glUniform2uiv: Option<glUniform2uiv_t>,
  glUniform3d: Option<glUniform3d_t>,
  glUniform3dv: Option<glUniform3dv_t>,
  glUniform3f: Option<glUniform3f_t>,
  glUniform3fv: Option<glUniform3fv_t>,
  glUniform3i: Option<glUniform3i_t>,
  glUniform3iv: Option<glUniform3iv_t>,
  glUniform3ui: Option<glUniform3ui_t>,
  glUniform3uiv: Option<glUniform3uiv_t>,
  glUniform4d: Option<glUniform4d_t>,
  glUniform4dv: Option<glUniform4dv_t>,
  glUniform4f: Option<glUniform4f_t>,
  glUniform4fv: Option<glUniform4fv_t>,
  glUniform4i: Option<glUniform4i_t>,
  glUniform4iv: Option<glUniform4iv_t>,
  glUniform4ui: Option<glUniform4ui_t>,
  glUniform4uiv: Option<glUniform4uiv_t>,
  glUniformBlockBinding: Option<glUniformBlockBinding_t>,
  glUniformMatrix2dv: Option<glUniformMatrix2dv_t>,
  glUniformMatrix2fv: Option<glUniformMatrix2fv_t>,
  glUniformMatrix2x3dv: Option<glUniformMatrix2x3dv_t>,
  glUniformMatrix2x3fv: Option<glUniformMatrix2x3fv_t>,
  glUniformMatrix2x4dv: Option<glUniformMatrix2x4dv_t>,
  glUniformMatrix2x4fv: Option<glUniformMatrix2x4fv_t>,
  glUniformMatrix3dv: Option<glUniformMatrix3dv_t>,
  glUniformMatrix3fv: Option<glUniformMatrix3fv_t>,
  glUniformMatrix3x2dv: Option<glUniformMatrix3x2dv_t>,
  glUniformMatrix3x2fv: Option<glUniformMatrix3x2fv_t>,
  glUniformMatrix3x4dv: Option<glUniformMatrix3x4dv_t>,
  glUniformMatrix3x4fv: Option<glUniformMatrix3x4fv_t>,
  glUniformMatrix4dv: Option<glUniformMatrix4dv_t>,
  glUniformMatrix4fv: Option<glUniformMatrix4fv_t>,
  glUniformMatrix4x2dv: Option<glUniformMatrix4x2dv_t>,
  glUniformMatrix4x2fv: Option<glUniformMatrix4x2fv_t>,
  glUniformMatrix4x3dv: Option<glUniformMatrix4x3dv_t>,
  glUniformMatrix4x3fv: Option<glUniformMatrix4x3fv_t>,
  glUniformSubroutinesuiv: Option<glUniformSubroutinesuiv_t>,
  glUnmapBuffer: Option<glUnmapBuffer_t>,
  glUnmapNamedBuffer: Option<glUnmapNamedBuffer_t>,
  glUseProgram: Option<glUseProgram_t>,
  glUseProgramStages: Option<glUseProgramStages_t>,
  glValidateProgram: Option<glValidateProgram_t>,
  glValidateProgramPipeline: Option<glValidateProgramPipeline_t>,
  glVertexArrayAttribBinding: Option<glVertexArrayAttribBinding_t>,
  glVertexArrayAttribFormat: Option<glVertexArrayAttribFormat_t>,
  glVertexArrayAttribIFormat: Option<glVertexArrayAttribIFormat_t>,
  glVertexArrayAttribLFormat: Option<glVertexArrayAttribLFormat_t>,
  glVertexArrayBindingDivisor: Option<glVertexArrayBindingDivisor_t>,
  glVertexArrayElementBuffer: Option<glVertexArrayElementBuffer_t>,
  glVertexArrayVertexBuffer: Option<glVertexArrayVertexBuffer_t>,
  glVertexArrayVertexBuffers: Option<glVertexArrayVertexBuffers_t>,
  glVertexAttrib1d: Option<glVertexAttrib1d_t>,
  glVertexAttrib1dv: Option<glVertexAttrib1dv_t>,
  glVertexAttrib1f: Option<glVertexAttrib1f_t>,
  glVertexAttrib1fv: Option<glVertexAttrib1fv_t>,
  glVertexAttrib1s: Option<glVertexAttrib1s_t>,
  glVertexAttrib1sv: Option<glVertexAttrib1sv_t>,
  glVertexAttrib2d: Option<glVertexAttrib2d_t>,
  glVertexAttrib2dv: Option<glVertexAttrib2dv_t>,
  glVertexAttrib2f: Option<glVertexAttrib2f_t>,
  glVertexAttrib2fv: Option<glVertexAttrib2fv_t>,
  glVertexAttrib2s: Option<glVertexAttrib2s_t>,
  glVertexAttrib2sv: Option<glVertexAttrib2sv_t>,
  glVertexAttrib3d: Option<glVertexAttrib3d_t>,
  glVertexAttrib3dv: Option<glVertexAttrib3dv_t>,
  glVertexAttrib3f: Option<glVertexAttrib3f_t>,
  glVertexAttrib3fv: Option<glVertexAttrib3fv_t>,
  glVertexAttrib3s: Option<glVertexAttrib3s_t>,
  glVertexAttrib3sv: Option<glVertexAttrib3sv_t>,
  glVertexAttrib4Nbv: Option<glVertexAttrib4Nbv_t>,
  glVertexAttrib4Niv: Option<glVertexAttrib4Niv_t>,
  glVertexAttrib4Nsv: Option<glVertexAttrib4Nsv_t>,
  glVertexAttrib4Nub: Option<glVertexAttrib4Nub_t>,
  glVertexAttrib4Nubv: Option<glVertexAttrib4Nubv_t>,
  glVertexAttrib4Nuiv: Option<glVertexAttrib4Nuiv_t>,
  glVertexAttrib4Nusv: Option<glVertexAttrib4Nusv_t>,
  glVertexAttrib4bv: Option<glVertexAttrib4bv_t>,
  glVertexAttrib4d: Option<glVertexAttrib4d_t>,
  glVertexAttrib4dv: Option<glVertexAttrib4dv_t>,
  glVertexAttrib4f: Option<glVertexAttrib4f_t>,
  glVertexAttrib4fv: Option<glVertexAttrib4fv_t>,
  glVertexAttrib4iv: Option<glVertexAttrib4iv_t>,
  glVertexAttrib4s: Option<glVertexAttrib4s_t>,
  glVertexAttrib4sv: Option<glVertexAttrib4sv_t>,
  glVertexAttrib4ubv: Option<glVertexAttrib4ubv_t>,
  glVertexAttrib4uiv: Option<glVertexAttrib4uiv_t>,
  glVertexAttrib4usv: Option<glVertexAttrib4usv_t>,
  glVertexAttribBinding: Option<glVertexAttribBinding_t>,
  glVertexAttribDivisor: Option<glVertexAttribDivisor_t>,
  glVertexAttribFormat: Option<glVertexAttribFormat_t>,
  glVertexAttribI1i: Option<glVertexAttribI1i_t>,
  glVertexAttribI1iv: Option<glVertexAttribI1iv_t>,
  glVertexAttribI1ui: Option<glVertexAttribI1ui_t>,
  glVertexAttribI1uiv: Option<glVertexAttribI1uiv_t>,
  glVertexAttribI2i: Option<glVertexAttribI2i_t>,
  glVertexAttribI2iv: Option<glVertexAttribI2iv_t>,
  glVertexAttribI2ui: Option<glVertexAttribI2ui_t>,
  glVertexAttribI2uiv: Option<glVertexAttribI2uiv_t>,
  glVertexAttribI3i: Option<glVertexAttribI3i_t>,
  glVertexAttribI3iv: Option<glVertexAttribI3iv_t>,
  glVertexAttribI3ui: Option<glVertexAttribI3ui_t>,
  glVertexAttribI3uiv: Option<glVertexAttribI3uiv_t>,
  glVertexAttribI4bv: Option<glVertexAttribI4bv_t>,
  glVertexAttribI4i: Option<glVertexAttribI4i_t>,
  glVertexAttribI4iv: Option<glVertexAttribI4iv_t>,
  glVertexAttribI4sv: Option<glVertexAttribI4sv_t>,
  glVertexAttribI4ubv: Option<glVertexAttribI4ubv_t>,
  glVertexAttribI4ui: Option<glVertexAttribI4ui_t>,
  glVertexAttribI4uiv: Option<glVertexAttribI4uiv_t>,
  glVertexAttribI4usv: Option<glVertexAttribI4usv_t>,
  glVertexAttribIFormat: Option<glVertexAttribIFormat_t>,
  glVertexAttribIPointer: Option<glVertexAttribIPointer_t>,
  glVertexAttribL1d: Option<glVertexAttribL1d_t>,
  glVertexAttribL1dv: Option<glVertexAttribL1dv_t>,
  glVertexAttribL2d: Option<glVertexAttribL2d_t>,
  glVertexAttribL2dv: Option<glVertexAttribL2dv_t>,
  glVertexAttribL3d: Option<glVertexAttribL3d_t>,
  glVertexAttribL3dv: Option<glVertexAttribL3dv_t>,
  glVertexAttribL4d: Option<glVertexAttribL4d_t>,
  glVertexAttribL4dv: Option<glVertexAttribL4dv_t>,
  glVertexAttribLFormat: Option<glVertexAttribLFormat_t>,
  glVertexAttribLPointer: Option<glVertexAttribLPointer_t>,
  glVertexAttribP1ui: Option<glVertexAttribP1ui_t>,
  glVertexAttribP1uiv: Option<glVertexAttribP1uiv_t>,
  glVertexAttribP2ui: Option<glVertexAttribP2ui_t>,
  glVertexAttribP2uiv: Option<glVertexAttribP2uiv_t>,
  glVertexAttribP3ui: Option<glVertexAttribP3ui_t>,
  glVertexAttribP3uiv: Option<glVertexAttribP3uiv_t>,
  glVertexAttribP4ui: Option<glVertexAttribP4ui_t>,
  glVertexAttribP4uiv: Option<glVertexAttribP4uiv_t>,
  glVertexAttribPointer: Option<glVertexAttribPointer_t>,
  glVertexBindingDivisor: Option<glVertexBindingDivisor_t>,
  glVertexP2ui: Option<glVertexP2ui_t>,
  glVertexP2uiv: Option<glVertexP2uiv_t>,
  glVertexP3ui: Option<glVertexP3ui_t>,
  glVertexP3uiv: Option<glVertexP3uiv_t>,
  glVertexP4ui: Option<glVertexP4ui_t>,
  glVertexP4uiv: Option<glVertexP4uiv_t>,
  glViewport: Option<glViewport_t>,
  glViewportArrayv: Option<glViewportArrayv_t>,
  glViewportIndexedf: Option<glViewportIndexedf_t>,
  glViewportIndexedfv: Option<glViewportIndexedfv_t>,
  glWaitSync: Option<glWaitSync_t>,
}
impl GlFns {
  #[inline]
  #[must_use]
  pub fn new_boxed() -> Box<Self> {
    // this struct is usually *hundreds* of `usize` big,
    // so we do this heap dance to avoid stack strain.
    assert!(core::mem::size_of::<Self>() % core::mem::size_of::<Option<fn()>>() == 0);
    let len = core::mem::size_of::<Self>() / core::mem::size_of::<Option<fn()>>();
    let v: Vec<Option<fn()>> = vec![None; len];
    #[allow(clippy::type_complexity)]
    let b: Box<[Option<fn()>]> = v.into_boxed_slice();
    let ptr_slice: *mut [Option<fn()>] = Box::leak(b);
    let ptr_self: *mut Self = ptr_slice as *mut Self;
    let box_self: Box<Self> = unsafe { Box::from_raw(ptr_self) };
    box_self
  }
  #[allow(clippy::missing_safety_doc)]
  pub unsafe fn load(&mut self, mut loader: impl FnMut(*const u8) -> *const core::ffi::c_void) {
    use core::mem::transmute;
    self.glActiveShaderProgram = unsafe { transmute(loader("glActiveShaderProgram\0".as_ptr())) };
    self.glActiveTexture = unsafe { transmute(loader("glActiveTexture\0".as_ptr())) };
    self.glAttachShader = unsafe { transmute(loader("glAttachShader\0".as_ptr())) };
    self.glBeginConditionalRender =
      unsafe { transmute(loader("glBeginConditionalRender\0".as_ptr())) };
    self.glBeginQuery = unsafe { transmute(loader("glBeginQuery\0".as_ptr())) };
    self.glBeginQueryIndexed = unsafe { transmute(loader("glBeginQueryIndexed\0".as_ptr())) };
    self.glBeginTransformFeedback =
      unsafe { transmute(loader("glBeginTransformFeedback\0".as_ptr())) };
    self.glBindAttribLocation = unsafe { transmute(loader("glBindAttribLocation\0".as_ptr())) };
    self.glBindBuffer = unsafe { transmute(loader("glBindBuffer\0".as_ptr())) };
    self.glBindBufferBase = unsafe { transmute(loader("glBindBufferBase\0".as_ptr())) };
    self.glBindBufferRange = unsafe { transmute(loader("glBindBufferRange\0".as_ptr())) };
    self.glBindBuffersBase = unsafe { transmute(loader("glBindBuffersBase\0".as_ptr())) };
    self.glBindBuffersRange = unsafe { transmute(loader("glBindBuffersRange\0".as_ptr())) };
    self.glBindFragDataLocation = unsafe { transmute(loader("glBindFragDataLocation\0".as_ptr())) };
    self.glBindFragDataLocationIndexed =
      unsafe { transmute(loader("glBindFragDataLocationIndexed\0".as_ptr())) };
    self.glBindFramebuffer = unsafe { transmute(loader("glBindFramebuffer\0".as_ptr())) };
    self.glBindImageTexture = unsafe { transmute(loader("glBindImageTexture\0".as_ptr())) };
    self.glBindImageTextures = unsafe { transmute(loader("glBindImageTextures\0".as_ptr())) };
    self.glBindProgramPipeline = unsafe { transmute(loader("glBindProgramPipeline\0".as_ptr())) };
    self.glBindRenderbuffer = unsafe { transmute(loader("glBindRenderbuffer\0".as_ptr())) };
    self.glBindSampler = unsafe { transmute(loader("glBindSampler\0".as_ptr())) };
    self.glBindSamplers = unsafe { transmute(loader("glBindSamplers\0".as_ptr())) };
    self.glBindTexture = unsafe { transmute(loader("glBindTexture\0".as_ptr())) };
    self.glBindTextureUnit = unsafe { transmute(loader("glBindTextureUnit\0".as_ptr())) };
    self.glBindTextures = unsafe { transmute(loader("glBindTextures\0".as_ptr())) };
    self.glBindTransformFeedback =
      unsafe { transmute(loader("glBindTransformFeedback\0".as_ptr())) };
    self.glBindVertexArray = unsafe { transmute(loader("glBindVertexArray\0".as_ptr())) };
    self.glBindVertexBuffer = unsafe { transmute(loader("glBindVertexBuffer\0".as_ptr())) };
    self.glBindVertexBuffers = unsafe { transmute(loader("glBindVertexBuffers\0".as_ptr())) };
    self.glBlendBarrier = unsafe { transmute(loader("glBlendBarrier\0".as_ptr())) };
    self.glBlendColor = unsafe { transmute(loader("glBlendColor\0".as_ptr())) };
    self.glBlendEquation = unsafe { transmute(loader("glBlendEquation\0".as_ptr())) };
    self.glBlendEquationSeparate =
      unsafe { transmute(loader("glBlendEquationSeparate\0".as_ptr())) };
    self.glBlendEquationSeparatei =
      unsafe { transmute(loader("glBlendEquationSeparatei\0".as_ptr())) };
    self.glBlendEquationi = unsafe { transmute(loader("glBlendEquationi\0".as_ptr())) };
    self.glBlendFunc = unsafe { transmute(loader("glBlendFunc\0".as_ptr())) };
    self.glBlendFuncSeparate = unsafe { transmute(loader("glBlendFuncSeparate\0".as_ptr())) };
    self.glBlendFuncSeparatei = unsafe { transmute(loader("glBlendFuncSeparatei\0".as_ptr())) };
    self.glBlendFunci = unsafe { transmute(loader("glBlendFunci\0".as_ptr())) };
    self.glBlitFramebuffer = unsafe { transmute(loader("glBlitFramebuffer\0".as_ptr())) };
    self.glBlitNamedFramebuffer = unsafe { transmute(loader("glBlitNamedFramebuffer\0".as_ptr())) };
    self.glBufferData = unsafe { transmute(loader("glBufferData\0".as_ptr())) };
    self.glBufferStorage = unsafe { transmute(loader("glBufferStorage\0".as_ptr())) };
    self.glBufferSubData = unsafe { transmute(loader("glBufferSubData\0".as_ptr())) };
    self.glCheckFramebufferStatus =
      unsafe { transmute(loader("glCheckFramebufferStatus\0".as_ptr())) };
    self.glCheckNamedFramebufferStatus =
      unsafe { transmute(loader("glCheckNamedFramebufferStatus\0".as_ptr())) };
    self.glClampColor = unsafe { transmute(loader("glClampColor\0".as_ptr())) };
    self.glClear = unsafe { transmute(loader("glClear\0".as_ptr())) };
    self.glClearBufferData = unsafe { transmute(loader("glClearBufferData\0".as_ptr())) };
    self.glClearBufferSubData = unsafe { transmute(loader("glClearBufferSubData\0".as_ptr())) };
    self.glClearBufferfi = unsafe { transmute(loader("glClearBufferfi\0".as_ptr())) };
    self.glClearBufferfv = unsafe { transmute(loader("glClearBufferfv\0".as_ptr())) };
    self.glClearBufferiv = unsafe { transmute(loader("glClearBufferiv\0".as_ptr())) };
    self.glClearBufferuiv = unsafe { transmute(loader("glClearBufferuiv\0".as_ptr())) };
    self.glClearColor = unsafe { transmute(loader("glClearColor\0".as_ptr())) };
    self.glClearDepth = unsafe { transmute(loader("glClearDepth\0".as_ptr())) };
    self.glClearDepthf = unsafe { transmute(loader("glClearDepthf\0".as_ptr())) };
    self.glClearNamedBufferData = unsafe { transmute(loader("glClearNamedBufferData\0".as_ptr())) };
    self.glClearNamedBufferSubData =
      unsafe { transmute(loader("glClearNamedBufferSubData\0".as_ptr())) };
    self.glClearNamedFramebufferfi =
      unsafe { transmute(loader("glClearNamedFramebufferfi\0".as_ptr())) };
    self.glClearNamedFramebufferfv =
      unsafe { transmute(loader("glClearNamedFramebufferfv\0".as_ptr())) };
    self.glClearNamedFramebufferiv =
      unsafe { transmute(loader("glClearNamedFramebufferiv\0".as_ptr())) };
    self.glClearNamedFramebufferuiv =
      unsafe { transmute(loader("glClearNamedFramebufferuiv\0".as_ptr())) };
    self.glClearStencil = unsafe { transmute(loader("glClearStencil\0".as_ptr())) };
    self.glClearTexImage = unsafe { transmute(loader("glClearTexImage\0".as_ptr())) };
    self.glClearTexSubImage = unsafe { transmute(loader("glClearTexSubImage\0".as_ptr())) };
    self.glClientWaitSync = unsafe { transmute(loader("glClientWaitSync\0".as_ptr())) };
    self.glClipControl = unsafe { transmute(loader("glClipControl\0".as_ptr())) };
    self.glColorMask = unsafe { transmute(loader("glColorMask\0".as_ptr())) };
    self.glColorMaski = unsafe { transmute(loader("glColorMaski\0".as_ptr())) };
    self.glColorP3ui = unsafe { transmute(loader("glColorP3ui\0".as_ptr())) };
    self.glColorP3uiv = unsafe { transmute(loader("glColorP3uiv\0".as_ptr())) };
    self.glColorP4ui = unsafe { transmute(loader("glColorP4ui\0".as_ptr())) };
    self.glColorP4uiv = unsafe { transmute(loader("glColorP4uiv\0".as_ptr())) };
    self.glCompileShader = unsafe { transmute(loader("glCompileShader\0".as_ptr())) };
    self.glCompressedTexImage1D = unsafe { transmute(loader("glCompressedTexImage1D\0".as_ptr())) };
    self.glCompressedTexImage2D = unsafe { transmute(loader("glCompressedTexImage2D\0".as_ptr())) };
    self.glCompressedTexImage3D = unsafe { transmute(loader("glCompressedTexImage3D\0".as_ptr())) };
    self.glCompressedTexSubImage1D =
      unsafe { transmute(loader("glCompressedTexSubImage1D\0".as_ptr())) };
    self.glCompressedTexSubImage2D =
      unsafe { transmute(loader("glCompressedTexSubImage2D\0".as_ptr())) };
    self.glCompressedTexSubImage3D =
      unsafe { transmute(loader("glCompressedTexSubImage3D\0".as_ptr())) };
    self.glCompressedTextureSubImage1D =
      unsafe { transmute(loader("glCompressedTextureSubImage1D\0".as_ptr())) };
    self.glCompressedTextureSubImage2D =
      unsafe { transmute(loader("glCompressedTextureSubImage2D\0".as_ptr())) };
    self.glCompressedTextureSubImage3D =
      unsafe { transmute(loader("glCompressedTextureSubImage3D\0".as_ptr())) };
    self.glCopyBufferSubData = unsafe { transmute(loader("glCopyBufferSubData\0".as_ptr())) };
    self.glCopyImageSubData = unsafe { transmute(loader("glCopyImageSubData\0".as_ptr())) };
    self.glCopyNamedBufferSubData =
      unsafe { transmute(loader("glCopyNamedBufferSubData\0".as_ptr())) };
    self.glCopyTexImage1D = unsafe { transmute(loader("glCopyTexImage1D\0".as_ptr())) };
    self.glCopyTexImage2D = unsafe { transmute(loader("glCopyTexImage2D\0".as_ptr())) };
    self.glCopyTexSubImage1D = unsafe { transmute(loader("glCopyTexSubImage1D\0".as_ptr())) };
    self.glCopyTexSubImage2D = unsafe { transmute(loader("glCopyTexSubImage2D\0".as_ptr())) };
    self.glCopyTexSubImage3D = unsafe { transmute(loader("glCopyTexSubImage3D\0".as_ptr())) };
    self.glCopyTextureSubImage1D =
      unsafe { transmute(loader("glCopyTextureSubImage1D\0".as_ptr())) };
    self.glCopyTextureSubImage2D =
      unsafe { transmute(loader("glCopyTextureSubImage2D\0".as_ptr())) };
    self.glCopyTextureSubImage3D =
      unsafe { transmute(loader("glCopyTextureSubImage3D\0".as_ptr())) };
    self.glCreateBuffers = unsafe { transmute(loader("glCreateBuffers\0".as_ptr())) };
    self.glCreateFramebuffers = unsafe { transmute(loader("glCreateFramebuffers\0".as_ptr())) };
    self.glCreateProgram = unsafe { transmute(loader("glCreateProgram\0".as_ptr())) };
    self.glCreateProgramPipelines =
      unsafe { transmute(loader("glCreateProgramPipelines\0".as_ptr())) };
    self.glCreateQueries = unsafe { transmute(loader("glCreateQueries\0".as_ptr())) };
    self.glCreateRenderbuffers = unsafe { transmute(loader("glCreateRenderbuffers\0".as_ptr())) };
    self.glCreateSamplers = unsafe { transmute(loader("glCreateSamplers\0".as_ptr())) };
    self.glCreateShader = unsafe { transmute(loader("glCreateShader\0".as_ptr())) };
    self.glCreateShaderProgramv = unsafe { transmute(loader("glCreateShaderProgramv\0".as_ptr())) };
    self.glCreateTextures = unsafe { transmute(loader("glCreateTextures\0".as_ptr())) };
    self.glCreateTransformFeedbacks =
      unsafe { transmute(loader("glCreateTransformFeedbacks\0".as_ptr())) };
    self.glCreateVertexArrays = unsafe { transmute(loader("glCreateVertexArrays\0".as_ptr())) };
    self.glCullFace = unsafe { transmute(loader("glCullFace\0".as_ptr())) };
    self.glDebugMessageCallback = unsafe { transmute(loader("glDebugMessageCallback\0".as_ptr())) };
    self.glDebugMessageCallbackKHR =
      unsafe { transmute(loader("glDebugMessageCallbackKHR\0".as_ptr())) };
    self.glDebugMessageControl = unsafe { transmute(loader("glDebugMessageControl\0".as_ptr())) };
    self.glDebugMessageControlKHR =
      unsafe { transmute(loader("glDebugMessageControlKHR\0".as_ptr())) };
    self.glDebugMessageInsert = unsafe { transmute(loader("glDebugMessageInsert\0".as_ptr())) };
    self.glDebugMessageInsertKHR =
      unsafe { transmute(loader("glDebugMessageInsertKHR\0".as_ptr())) };
    self.glDeleteBuffers = unsafe { transmute(loader("glDeleteBuffers\0".as_ptr())) };
    self.glDeleteFramebuffers = unsafe { transmute(loader("glDeleteFramebuffers\0".as_ptr())) };
    self.glDeleteProgram = unsafe { transmute(loader("glDeleteProgram\0".as_ptr())) };
    self.glDeleteProgramPipelines =
      unsafe { transmute(loader("glDeleteProgramPipelines\0".as_ptr())) };
    self.glDeleteQueries = unsafe { transmute(loader("glDeleteQueries\0".as_ptr())) };
    self.glDeleteRenderbuffers = unsafe { transmute(loader("glDeleteRenderbuffers\0".as_ptr())) };
    self.glDeleteSamplers = unsafe { transmute(loader("glDeleteSamplers\0".as_ptr())) };
    self.glDeleteShader = unsafe { transmute(loader("glDeleteShader\0".as_ptr())) };
    self.glDeleteSync = unsafe { transmute(loader("glDeleteSync\0".as_ptr())) };
    self.glDeleteTextures = unsafe { transmute(loader("glDeleteTextures\0".as_ptr())) };
    self.glDeleteTransformFeedbacks =
      unsafe { transmute(loader("glDeleteTransformFeedbacks\0".as_ptr())) };
    self.glDeleteVertexArrays = unsafe { transmute(loader("glDeleteVertexArrays\0".as_ptr())) };
    self.glDepthFunc = unsafe { transmute(loader("glDepthFunc\0".as_ptr())) };
    self.glDepthMask = unsafe { transmute(loader("glDepthMask\0".as_ptr())) };
    self.glDepthRange = unsafe { transmute(loader("glDepthRange\0".as_ptr())) };
    self.glDepthRangeArrayv = unsafe { transmute(loader("glDepthRangeArrayv\0".as_ptr())) };
    self.glDepthRangeIndexed = unsafe { transmute(loader("glDepthRangeIndexed\0".as_ptr())) };
    self.glDepthRangef = unsafe { transmute(loader("glDepthRangef\0".as_ptr())) };
    self.glDetachShader = unsafe { transmute(loader("glDetachShader\0".as_ptr())) };
    self.glDisable = unsafe { transmute(loader("glDisable\0".as_ptr())) };
    self.glDisableVertexArrayAttrib =
      unsafe { transmute(loader("glDisableVertexArrayAttrib\0".as_ptr())) };
    self.glDisableVertexAttribArray =
      unsafe { transmute(loader("glDisableVertexAttribArray\0".as_ptr())) };
    self.glDisablei = unsafe { transmute(loader("glDisablei\0".as_ptr())) };
    self.glDispatchCompute = unsafe { transmute(loader("glDispatchCompute\0".as_ptr())) };
    self.glDispatchComputeIndirect =
      unsafe { transmute(loader("glDispatchComputeIndirect\0".as_ptr())) };
    self.glDrawArrays = unsafe { transmute(loader("glDrawArrays\0".as_ptr())) };
    self.glDrawArraysIndirect = unsafe { transmute(loader("glDrawArraysIndirect\0".as_ptr())) };
    self.glDrawArraysInstanced = unsafe { transmute(loader("glDrawArraysInstanced\0".as_ptr())) };
    self.glDrawArraysInstancedBaseInstance =
      unsafe { transmute(loader("glDrawArraysInstancedBaseInstance\0".as_ptr())) };
    self.glDrawBuffer = unsafe { transmute(loader("glDrawBuffer\0".as_ptr())) };
    self.glDrawBuffers = unsafe { transmute(loader("glDrawBuffers\0".as_ptr())) };
    self.glDrawElements = unsafe { transmute(loader("glDrawElements\0".as_ptr())) };
    self.glDrawElementsBaseVertex =
      unsafe { transmute(loader("glDrawElementsBaseVertex\0".as_ptr())) };
    self.glDrawElementsIndirect = unsafe { transmute(loader("glDrawElementsIndirect\0".as_ptr())) };
    self.glDrawElementsInstanced =
      unsafe { transmute(loader("glDrawElementsInstanced\0".as_ptr())) };
    self.glDrawElementsInstancedBaseInstance =
      unsafe { transmute(loader("glDrawElementsInstancedBaseInstance\0".as_ptr())) };
    self.glDrawElementsInstancedBaseVertex =
      unsafe { transmute(loader("glDrawElementsInstancedBaseVertex\0".as_ptr())) };
    self.glDrawElementsInstancedBaseVertexBaseInstance =
      unsafe { transmute(loader("glDrawElementsInstancedBaseVertexBaseInstance\0".as_ptr())) };
    self.glDrawRangeElements = unsafe { transmute(loader("glDrawRangeElements\0".as_ptr())) };
    self.glDrawRangeElementsBaseVertex =
      unsafe { transmute(loader("glDrawRangeElementsBaseVertex\0".as_ptr())) };
    self.glDrawTransformFeedback =
      unsafe { transmute(loader("glDrawTransformFeedback\0".as_ptr())) };
    self.glDrawTransformFeedbackInstanced =
      unsafe { transmute(loader("glDrawTransformFeedbackInstanced\0".as_ptr())) };
    self.glDrawTransformFeedbackStream =
      unsafe { transmute(loader("glDrawTransformFeedbackStream\0".as_ptr())) };
    self.glDrawTransformFeedbackStreamInstanced =
      unsafe { transmute(loader("glDrawTransformFeedbackStreamInstanced\0".as_ptr())) };
    self.glEnable = unsafe { transmute(loader("glEnable\0".as_ptr())) };
    self.glEnableVertexArrayAttrib =
      unsafe { transmute(loader("glEnableVertexArrayAttrib\0".as_ptr())) };
    self.glEnableVertexAttribArray =
      unsafe { transmute(loader("glEnableVertexAttribArray\0".as_ptr())) };
    self.glEnablei = unsafe { transmute(loader("glEnablei\0".as_ptr())) };
    self.glEndConditionalRender = unsafe { transmute(loader("glEndConditionalRender\0".as_ptr())) };
    self.glEndQuery = unsafe { transmute(loader("glEndQuery\0".as_ptr())) };
    self.glEndQueryIndexed = unsafe { transmute(loader("glEndQueryIndexed\0".as_ptr())) };
    self.glEndTransformFeedback = unsafe { transmute(loader("glEndTransformFeedback\0".as_ptr())) };
    self.glFenceSync = unsafe { transmute(loader("glFenceSync\0".as_ptr())) };
    self.glFinish = unsafe { transmute(loader("glFinish\0".as_ptr())) };
    self.glFlush = unsafe { transmute(loader("glFlush\0".as_ptr())) };
    self.glFlushMappedBufferRange =
      unsafe { transmute(loader("glFlushMappedBufferRange\0".as_ptr())) };
    self.glFlushMappedNamedBufferRange =
      unsafe { transmute(loader("glFlushMappedNamedBufferRange\0".as_ptr())) };
    self.glFramebufferParameteri =
      unsafe { transmute(loader("glFramebufferParameteri\0".as_ptr())) };
    self.glFramebufferRenderbuffer =
      unsafe { transmute(loader("glFramebufferRenderbuffer\0".as_ptr())) };
    self.glFramebufferTexture = unsafe { transmute(loader("glFramebufferTexture\0".as_ptr())) };
    self.glFramebufferTexture1D = unsafe { transmute(loader("glFramebufferTexture1D\0".as_ptr())) };
    self.glFramebufferTexture2D = unsafe { transmute(loader("glFramebufferTexture2D\0".as_ptr())) };
    self.glFramebufferTexture3D = unsafe { transmute(loader("glFramebufferTexture3D\0".as_ptr())) };
    self.glFramebufferTextureLayer =
      unsafe { transmute(loader("glFramebufferTextureLayer\0".as_ptr())) };
    self.glFrontFace = unsafe { transmute(loader("glFrontFace\0".as_ptr())) };
    self.glGenBuffers = unsafe { transmute(loader("glGenBuffers\0".as_ptr())) };
    self.glGenFramebuffers = unsafe { transmute(loader("glGenFramebuffers\0".as_ptr())) };
    self.glGenProgramPipelines = unsafe { transmute(loader("glGenProgramPipelines\0".as_ptr())) };
    self.glGenQueries = unsafe { transmute(loader("glGenQueries\0".as_ptr())) };
    self.glGenRenderbuffers = unsafe { transmute(loader("glGenRenderbuffers\0".as_ptr())) };
    self.glGenSamplers = unsafe { transmute(loader("glGenSamplers\0".as_ptr())) };
    self.glGenTextures = unsafe { transmute(loader("glGenTextures\0".as_ptr())) };
    self.glGenTransformFeedbacks =
      unsafe { transmute(loader("glGenTransformFeedbacks\0".as_ptr())) };
    self.glGenVertexArrays = unsafe { transmute(loader("glGenVertexArrays\0".as_ptr())) };
    self.glGenerateMipmap = unsafe { transmute(loader("glGenerateMipmap\0".as_ptr())) };
    self.glGenerateTextureMipmap =
      unsafe { transmute(loader("glGenerateTextureMipmap\0".as_ptr())) };
    self.glGetActiveAtomicCounterBufferiv =
      unsafe { transmute(loader("glGetActiveAtomicCounterBufferiv\0".as_ptr())) };
    self.glGetActiveAttrib = unsafe { transmute(loader("glGetActiveAttrib\0".as_ptr())) };
    self.glGetActiveSubroutineName =
      unsafe { transmute(loader("glGetActiveSubroutineName\0".as_ptr())) };
    self.glGetActiveSubroutineUniformName =
      unsafe { transmute(loader("glGetActiveSubroutineUniformName\0".as_ptr())) };
    self.glGetActiveSubroutineUniformiv =
      unsafe { transmute(loader("glGetActiveSubroutineUniformiv\0".as_ptr())) };
    self.glGetActiveUniform = unsafe { transmute(loader("glGetActiveUniform\0".as_ptr())) };
    self.glGetActiveUniformBlockName =
      unsafe { transmute(loader("glGetActiveUniformBlockName\0".as_ptr())) };
    self.glGetActiveUniformBlockiv =
      unsafe { transmute(loader("glGetActiveUniformBlockiv\0".as_ptr())) };
    self.glGetActiveUniformName = unsafe { transmute(loader("glGetActiveUniformName\0".as_ptr())) };
    self.glGetActiveUniformsiv = unsafe { transmute(loader("glGetActiveUniformsiv\0".as_ptr())) };
    self.glGetAttachedShaders = unsafe { transmute(loader("glGetAttachedShaders\0".as_ptr())) };
    self.glGetAttribLocation = unsafe { transmute(loader("glGetAttribLocation\0".as_ptr())) };
    self.glGetBooleani_v = unsafe { transmute(loader("glGetBooleani_v\0".as_ptr())) };
    self.glGetBooleanv = unsafe { transmute(loader("glGetBooleanv\0".as_ptr())) };
    self.glGetBufferParameteri64v =
      unsafe { transmute(loader("glGetBufferParameteri64v\0".as_ptr())) };
    self.glGetBufferParameteriv = unsafe { transmute(loader("glGetBufferParameteriv\0".as_ptr())) };
    self.glGetBufferPointerv = unsafe { transmute(loader("glGetBufferPointerv\0".as_ptr())) };
    self.glGetBufferSubData = unsafe { transmute(loader("glGetBufferSubData\0".as_ptr())) };
    self.glGetCompressedTexImage =
      unsafe { transmute(loader("glGetCompressedTexImage\0".as_ptr())) };
    self.glGetCompressedTextureImage =
      unsafe { transmute(loader("glGetCompressedTextureImage\0".as_ptr())) };
    self.glGetCompressedTextureSubImage =
      unsafe { transmute(loader("glGetCompressedTextureSubImage\0".as_ptr())) };
    self.glGetDebugMessageLog = unsafe { transmute(loader("glGetDebugMessageLog\0".as_ptr())) };
    self.glGetDebugMessageLogKHR =
      unsafe { transmute(loader("glGetDebugMessageLogKHR\0".as_ptr())) };
    self.glGetDoublei_v = unsafe { transmute(loader("glGetDoublei_v\0".as_ptr())) };
    self.glGetDoublev = unsafe { transmute(loader("glGetDoublev\0".as_ptr())) };
    self.glGetError = unsafe { transmute(loader("glGetError\0".as_ptr())) };
    self.glGetFloati_v = unsafe { transmute(loader("glGetFloati_v\0".as_ptr())) };
    self.glGetFloatv = unsafe { transmute(loader("glGetFloatv\0".as_ptr())) };
    self.glGetFragDataIndex = unsafe { transmute(loader("glGetFragDataIndex\0".as_ptr())) };
    self.glGetFragDataLocation = unsafe { transmute(loader("glGetFragDataLocation\0".as_ptr())) };
    self.glGetFramebufferAttachmentParameteriv =
      unsafe { transmute(loader("glGetFramebufferAttachmentParameteriv\0".as_ptr())) };
    self.glGetFramebufferParameteriv =
      unsafe { transmute(loader("glGetFramebufferParameteriv\0".as_ptr())) };
    self.glGetGraphicsResetStatus =
      unsafe { transmute(loader("glGetGraphicsResetStatus\0".as_ptr())) };
    self.glGetInteger64i_v = unsafe { transmute(loader("glGetInteger64i_v\0".as_ptr())) };
    self.glGetInteger64v = unsafe { transmute(loader("glGetInteger64v\0".as_ptr())) };
    self.glGetIntegeri_v = unsafe { transmute(loader("glGetIntegeri_v\0".as_ptr())) };
    self.glGetIntegerv = unsafe { transmute(loader("glGetIntegerv\0".as_ptr())) };
    self.glGetInternalformati64v =
      unsafe { transmute(loader("glGetInternalformati64v\0".as_ptr())) };
    self.glGetInternalformativ = unsafe { transmute(loader("glGetInternalformativ\0".as_ptr())) };
    self.glGetMultisamplefv = unsafe { transmute(loader("glGetMultisamplefv\0".as_ptr())) };
    self.glGetNamedBufferParameteri64v =
      unsafe { transmute(loader("glGetNamedBufferParameteri64v\0".as_ptr())) };
    self.glGetNamedBufferParameteriv =
      unsafe { transmute(loader("glGetNamedBufferParameteriv\0".as_ptr())) };
    self.glGetNamedBufferPointerv =
      unsafe { transmute(loader("glGetNamedBufferPointerv\0".as_ptr())) };
    self.glGetNamedBufferSubData =
      unsafe { transmute(loader("glGetNamedBufferSubData\0".as_ptr())) };
    self.glGetNamedFramebufferAttachmentParameteriv =
      unsafe { transmute(loader("glGetNamedFramebufferAttachmentParameteriv\0".as_ptr())) };
    self.glGetNamedFramebufferParameteriv =
      unsafe { transmute(loader("glGetNamedFramebufferParameteriv\0".as_ptr())) };
    self.glGetNamedRenderbufferParameteriv =
      unsafe { transmute(loader("glGetNamedRenderbufferParameteriv\0".as_ptr())) };
    self.glGetObjectLabel = unsafe { transmute(loader("glGetObjectLabel\0".as_ptr())) };
    self.glGetObjectLabelKHR = unsafe { transmute(loader("glGetObjectLabelKHR\0".as_ptr())) };
    self.glGetObjectPtrLabel = unsafe { transmute(loader("glGetObjectPtrLabel\0".as_ptr())) };
    self.glGetObjectPtrLabelKHR = unsafe { transmute(loader("glGetObjectPtrLabelKHR\0".as_ptr())) };
    self.glGetPointerv = unsafe { transmute(loader("glGetPointerv\0".as_ptr())) };
    self.glGetPointervKHR = unsafe { transmute(loader("glGetPointervKHR\0".as_ptr())) };
    self.glGetProgramBinary = unsafe { transmute(loader("glGetProgramBinary\0".as_ptr())) };
    self.glGetProgramInfoLog = unsafe { transmute(loader("glGetProgramInfoLog\0".as_ptr())) };
    self.glGetProgramInterfaceiv =
      unsafe { transmute(loader("glGetProgramInterfaceiv\0".as_ptr())) };
    self.glGetProgramPipelineInfoLog =
      unsafe { transmute(loader("glGetProgramPipelineInfoLog\0".as_ptr())) };
    self.glGetProgramPipelineiv = unsafe { transmute(loader("glGetProgramPipelineiv\0".as_ptr())) };
    self.glGetProgramResourceIndex =
      unsafe { transmute(loader("glGetProgramResourceIndex\0".as_ptr())) };
    self.glGetProgramResourceLocation =
      unsafe { transmute(loader("glGetProgramResourceLocation\0".as_ptr())) };
    self.glGetProgramResourceLocationIndex =
      unsafe { transmute(loader("glGetProgramResourceLocationIndex\0".as_ptr())) };
    self.glGetProgramResourceName =
      unsafe { transmute(loader("glGetProgramResourceName\0".as_ptr())) };
    self.glGetProgramResourceiv = unsafe { transmute(loader("glGetProgramResourceiv\0".as_ptr())) };
    self.glGetProgramStageiv = unsafe { transmute(loader("glGetProgramStageiv\0".as_ptr())) };
    self.glGetProgramiv = unsafe { transmute(loader("glGetProgramiv\0".as_ptr())) };
    self.glGetQueryBufferObjecti64v =
      unsafe { transmute(loader("glGetQueryBufferObjecti64v\0".as_ptr())) };
    self.glGetQueryBufferObjectiv =
      unsafe { transmute(loader("glGetQueryBufferObjectiv\0".as_ptr())) };
    self.glGetQueryBufferObjectui64v =
      unsafe { transmute(loader("glGetQueryBufferObjectui64v\0".as_ptr())) };
    self.glGetQueryBufferObjectuiv =
      unsafe { transmute(loader("glGetQueryBufferObjectuiv\0".as_ptr())) };
    self.glGetQueryIndexediv = unsafe { transmute(loader("glGetQueryIndexediv\0".as_ptr())) };
    self.glGetQueryObjecti64v = unsafe { transmute(loader("glGetQueryObjecti64v\0".as_ptr())) };
    self.glGetQueryObjectiv = unsafe { transmute(loader("glGetQueryObjectiv\0".as_ptr())) };
    self.glGetQueryObjectui64v = unsafe { transmute(loader("glGetQueryObjectui64v\0".as_ptr())) };
    self.glGetQueryObjectuiv = unsafe { transmute(loader("glGetQueryObjectuiv\0".as_ptr())) };
    self.glGetQueryiv = unsafe { transmute(loader("glGetQueryiv\0".as_ptr())) };
    self.glGetRenderbufferParameteriv =
      unsafe { transmute(loader("glGetRenderbufferParameteriv\0".as_ptr())) };
    self.glGetSamplerParameterIiv =
      unsafe { transmute(loader("glGetSamplerParameterIiv\0".as_ptr())) };
    self.glGetSamplerParameterIuiv =
      unsafe { transmute(loader("glGetSamplerParameterIuiv\0".as_ptr())) };
    self.glGetSamplerParameterfv =
      unsafe { transmute(loader("glGetSamplerParameterfv\0".as_ptr())) };
    self.glGetSamplerParameteriv =
      unsafe { transmute(loader("glGetSamplerParameteriv\0".as_ptr())) };
    self.glGetShaderInfoLog = unsafe { transmute(loader("glGetShaderInfoLog\0".as_ptr())) };
    self.glGetShaderPrecisionFormat =
      unsafe { transmute(loader("glGetShaderPrecisionFormat\0".as_ptr())) };
    self.glGetShaderSource = unsafe { transmute(loader("glGetShaderSource\0".as_ptr())) };
    self.glGetShaderiv = unsafe { transmute(loader("glGetShaderiv\0".as_ptr())) };
    self.glGetString = unsafe { transmute(loader("glGetString\0".as_ptr())) };
    self.glGetStringi = unsafe { transmute(loader("glGetStringi\0".as_ptr())) };
    self.glGetSubroutineIndex = unsafe { transmute(loader("glGetSubroutineIndex\0".as_ptr())) };
    self.glGetSubroutineUniformLocation =
      unsafe { transmute(loader("glGetSubroutineUniformLocation\0".as_ptr())) };
    self.glGetSynciv = unsafe { transmute(loader("glGetSynciv\0".as_ptr())) };
    self.glGetTexImage = unsafe { transmute(loader("glGetTexImage\0".as_ptr())) };
    self.glGetTexLevelParameterfv =
      unsafe { transmute(loader("glGetTexLevelParameterfv\0".as_ptr())) };
    self.glGetTexLevelParameteriv =
      unsafe { transmute(loader("glGetTexLevelParameteriv\0".as_ptr())) };
    self.glGetTexParameterIiv = unsafe { transmute(loader("glGetTexParameterIiv\0".as_ptr())) };
    self.glGetTexParameterIuiv = unsafe { transmute(loader("glGetTexParameterIuiv\0".as_ptr())) };
    self.glGetTexParameterfv = unsafe { transmute(loader("glGetTexParameterfv\0".as_ptr())) };
    self.glGetTexParameteriv = unsafe { transmute(loader("glGetTexParameteriv\0".as_ptr())) };
    self.glGetTextureImage = unsafe { transmute(loader("glGetTextureImage\0".as_ptr())) };
    self.glGetTextureLevelParameterfv =
      unsafe { transmute(loader("glGetTextureLevelParameterfv\0".as_ptr())) };
    self.glGetTextureLevelParameteriv =
      unsafe { transmute(loader("glGetTextureLevelParameteriv\0".as_ptr())) };
    self.glGetTextureParameterIiv =
      unsafe { transmute(loader("glGetTextureParameterIiv\0".as_ptr())) };
    self.glGetTextureParameterIuiv =
      unsafe { transmute(loader("glGetTextureParameterIuiv\0".as_ptr())) };
    self.glGetTextureParameterfv =
      unsafe { transmute(loader("glGetTextureParameterfv\0".as_ptr())) };
    self.glGetTextureParameteriv =
      unsafe { transmute(loader("glGetTextureParameteriv\0".as_ptr())) };
    self.glGetTextureSubImage = unsafe { transmute(loader("glGetTextureSubImage\0".as_ptr())) };
    self.glGetTransformFeedbackVarying =
      unsafe { transmute(loader("glGetTransformFeedbackVarying\0".as_ptr())) };
    self.glGetTransformFeedbacki64_v =
      unsafe { transmute(loader("glGetTransformFeedbacki64_v\0".as_ptr())) };
    self.glGetTransformFeedbacki_v =
      unsafe { transmute(loader("glGetTransformFeedbacki_v\0".as_ptr())) };
    self.glGetTransformFeedbackiv =
      unsafe { transmute(loader("glGetTransformFeedbackiv\0".as_ptr())) };
    self.glGetUniformBlockIndex = unsafe { transmute(loader("glGetUniformBlockIndex\0".as_ptr())) };
    self.glGetUniformIndices = unsafe { transmute(loader("glGetUniformIndices\0".as_ptr())) };
    self.glGetUniformLocation = unsafe { transmute(loader("glGetUniformLocation\0".as_ptr())) };
    self.glGetUniformSubroutineuiv =
      unsafe { transmute(loader("glGetUniformSubroutineuiv\0".as_ptr())) };
    self.glGetUniformdv = unsafe { transmute(loader("glGetUniformdv\0".as_ptr())) };
    self.glGetUniformfv = unsafe { transmute(loader("glGetUniformfv\0".as_ptr())) };
    self.glGetUniformiv = unsafe { transmute(loader("glGetUniformiv\0".as_ptr())) };
    self.glGetUniformuiv = unsafe { transmute(loader("glGetUniformuiv\0".as_ptr())) };
    self.glGetVertexArrayIndexed64iv =
      unsafe { transmute(loader("glGetVertexArrayIndexed64iv\0".as_ptr())) };
    self.glGetVertexArrayIndexediv =
      unsafe { transmute(loader("glGetVertexArrayIndexediv\0".as_ptr())) };
    self.glGetVertexArrayiv = unsafe { transmute(loader("glGetVertexArrayiv\0".as_ptr())) };
    self.glGetVertexAttribIiv = unsafe { transmute(loader("glGetVertexAttribIiv\0".as_ptr())) };
    self.glGetVertexAttribIuiv = unsafe { transmute(loader("glGetVertexAttribIuiv\0".as_ptr())) };
    self.glGetVertexAttribLdv = unsafe { transmute(loader("glGetVertexAttribLdv\0".as_ptr())) };
    self.glGetVertexAttribPointerv =
      unsafe { transmute(loader("glGetVertexAttribPointerv\0".as_ptr())) };
    self.glGetVertexAttribdv = unsafe { transmute(loader("glGetVertexAttribdv\0".as_ptr())) };
    self.glGetVertexAttribfv = unsafe { transmute(loader("glGetVertexAttribfv\0".as_ptr())) };
    self.glGetVertexAttribiv = unsafe { transmute(loader("glGetVertexAttribiv\0".as_ptr())) };
    self.glGetnColorTable = unsafe { transmute(loader("glGetnColorTable\0".as_ptr())) };
    self.glGetnCompressedTexImage =
      unsafe { transmute(loader("glGetnCompressedTexImage\0".as_ptr())) };
    self.glGetnConvolutionFilter =
      unsafe { transmute(loader("glGetnConvolutionFilter\0".as_ptr())) };
    self.glGetnHistogram = unsafe { transmute(loader("glGetnHistogram\0".as_ptr())) };
    self.glGetnMapdv = unsafe { transmute(loader("glGetnMapdv\0".as_ptr())) };
    self.glGetnMapfv = unsafe { transmute(loader("glGetnMapfv\0".as_ptr())) };
    self.glGetnMapiv = unsafe { transmute(loader("glGetnMapiv\0".as_ptr())) };
    self.glGetnMinmax = unsafe { transmute(loader("glGetnMinmax\0".as_ptr())) };
    self.glGetnPixelMapfv = unsafe { transmute(loader("glGetnPixelMapfv\0".as_ptr())) };
    self.glGetnPixelMapuiv = unsafe { transmute(loader("glGetnPixelMapuiv\0".as_ptr())) };
    self.glGetnPixelMapusv = unsafe { transmute(loader("glGetnPixelMapusv\0".as_ptr())) };
    self.glGetnPolygonStipple = unsafe { transmute(loader("glGetnPolygonStipple\0".as_ptr())) };
    self.glGetnSeparableFilter = unsafe { transmute(loader("glGetnSeparableFilter\0".as_ptr())) };
    self.glGetnTexImage = unsafe { transmute(loader("glGetnTexImage\0".as_ptr())) };
    self.glGetnUniformdv = unsafe { transmute(loader("glGetnUniformdv\0".as_ptr())) };
    self.glGetnUniformfv = unsafe { transmute(loader("glGetnUniformfv\0".as_ptr())) };
    self.glGetnUniformiv = unsafe { transmute(loader("glGetnUniformiv\0".as_ptr())) };
    self.glGetnUniformuiv = unsafe { transmute(loader("glGetnUniformuiv\0".as_ptr())) };
    self.glHint = unsafe { transmute(loader("glHint\0".as_ptr())) };
    self.glInvalidateBufferData = unsafe { transmute(loader("glInvalidateBufferData\0".as_ptr())) };
    self.glInvalidateBufferSubData =
      unsafe { transmute(loader("glInvalidateBufferSubData\0".as_ptr())) };
    self.glInvalidateFramebuffer =
      unsafe { transmute(loader("glInvalidateFramebuffer\0".as_ptr())) };
    self.glInvalidateNamedFramebufferData =
      unsafe { transmute(loader("glInvalidateNamedFramebufferData\0".as_ptr())) };
    self.glInvalidateNamedFramebufferSubData =
      unsafe { transmute(loader("glInvalidateNamedFramebufferSubData\0".as_ptr())) };
    self.glInvalidateSubFramebuffer =
      unsafe { transmute(loader("glInvalidateSubFramebuffer\0".as_ptr())) };
    self.glInvalidateTexImage = unsafe { transmute(loader("glInvalidateTexImage\0".as_ptr())) };
    self.glInvalidateTexSubImage =
      unsafe { transmute(loader("glInvalidateTexSubImage\0".as_ptr())) };
    self.glIsBuffer = unsafe { transmute(loader("glIsBuffer\0".as_ptr())) };
    self.glIsEnabled = unsafe { transmute(loader("glIsEnabled\0".as_ptr())) };
    self.glIsEnabledi = unsafe { transmute(loader("glIsEnabledi\0".as_ptr())) };
    self.glIsFramebuffer = unsafe { transmute(loader("glIsFramebuffer\0".as_ptr())) };
    self.glIsProgram = unsafe { transmute(loader("glIsProgram\0".as_ptr())) };
    self.glIsProgramPipeline = unsafe { transmute(loader("glIsProgramPipeline\0".as_ptr())) };
    self.glIsQuery = unsafe { transmute(loader("glIsQuery\0".as_ptr())) };
    self.glIsRenderbuffer = unsafe { transmute(loader("glIsRenderbuffer\0".as_ptr())) };
    self.glIsSampler = unsafe { transmute(loader("glIsSampler\0".as_ptr())) };
    self.glIsShader = unsafe { transmute(loader("glIsShader\0".as_ptr())) };
    self.glIsSync = unsafe { transmute(loader("glIsSync\0".as_ptr())) };
    self.glIsTexture = unsafe { transmute(loader("glIsTexture\0".as_ptr())) };
    self.glIsTransformFeedback = unsafe { transmute(loader("glIsTransformFeedback\0".as_ptr())) };
    self.glIsVertexArray = unsafe { transmute(loader("glIsVertexArray\0".as_ptr())) };
    self.glLineWidth = unsafe { transmute(loader("glLineWidth\0".as_ptr())) };
    self.glLinkProgram = unsafe { transmute(loader("glLinkProgram\0".as_ptr())) };
    self.glLogicOp = unsafe { transmute(loader("glLogicOp\0".as_ptr())) };
    self.glMapBuffer = unsafe { transmute(loader("glMapBuffer\0".as_ptr())) };
    self.glMapBufferRange = unsafe { transmute(loader("glMapBufferRange\0".as_ptr())) };
    self.glMapNamedBuffer = unsafe { transmute(loader("glMapNamedBuffer\0".as_ptr())) };
    self.glMapNamedBufferRange = unsafe { transmute(loader("glMapNamedBufferRange\0".as_ptr())) };
    self.glMemoryBarrier = unsafe { transmute(loader("glMemoryBarrier\0".as_ptr())) };
    self.glMemoryBarrierByRegion =
      unsafe { transmute(loader("glMemoryBarrierByRegion\0".as_ptr())) };
    self.glMinSampleShading = unsafe { transmute(loader("glMinSampleShading\0".as_ptr())) };
    self.glMultiDrawArrays = unsafe { transmute(loader("glMultiDrawArrays\0".as_ptr())) };
    self.glMultiDrawArraysIndirect =
      unsafe { transmute(loader("glMultiDrawArraysIndirect\0".as_ptr())) };
    self.glMultiDrawArraysIndirectCount =
      unsafe { transmute(loader("glMultiDrawArraysIndirectCount\0".as_ptr())) };
    self.glMultiDrawElements = unsafe { transmute(loader("glMultiDrawElements\0".as_ptr())) };
    self.glMultiDrawElementsBaseVertex =
      unsafe { transmute(loader("glMultiDrawElementsBaseVertex\0".as_ptr())) };
    self.glMultiDrawElementsIndirect =
      unsafe { transmute(loader("glMultiDrawElementsIndirect\0".as_ptr())) };
    self.glMultiDrawElementsIndirectCount =
      unsafe { transmute(loader("glMultiDrawElementsIndirectCount\0".as_ptr())) };
    self.glMultiTexCoordP1ui = unsafe { transmute(loader("glMultiTexCoordP1ui\0".as_ptr())) };
    self.glMultiTexCoordP1uiv = unsafe { transmute(loader("glMultiTexCoordP1uiv\0".as_ptr())) };
    self.glMultiTexCoordP2ui = unsafe { transmute(loader("glMultiTexCoordP2ui\0".as_ptr())) };
    self.glMultiTexCoordP2uiv = unsafe { transmute(loader("glMultiTexCoordP2uiv\0".as_ptr())) };
    self.glMultiTexCoordP3ui = unsafe { transmute(loader("glMultiTexCoordP3ui\0".as_ptr())) };
    self.glMultiTexCoordP3uiv = unsafe { transmute(loader("glMultiTexCoordP3uiv\0".as_ptr())) };
    self.glMultiTexCoordP4ui = unsafe { transmute(loader("glMultiTexCoordP4ui\0".as_ptr())) };
    self.glMultiTexCoordP4uiv = unsafe { transmute(loader("glMultiTexCoordP4uiv\0".as_ptr())) };
    self.glNamedBufferData = unsafe { transmute(loader("glNamedBufferData\0".as_ptr())) };
    self.glNamedBufferStorage = unsafe { transmute(loader("glNamedBufferStorage\0".as_ptr())) };
    self.glNamedBufferSubData = unsafe { transmute(loader("glNamedBufferSubData\0".as_ptr())) };
    self.glNamedFramebufferDrawBuffer =
      unsafe { transmute(loader("glNamedFramebufferDrawBuffer\0".as_ptr())) };
    self.glNamedFramebufferDrawBuffers =
      unsafe { transmute(loader("glNamedFramebufferDrawBuffers\0".as_ptr())) };
    self.glNamedFramebufferParameteri =
      unsafe { transmute(loader("glNamedFramebufferParameteri\0".as_ptr())) };
    self.glNamedFramebufferReadBuffer =
      unsafe { transmute(loader("glNamedFramebufferReadBuffer\0".as_ptr())) };
    self.glNamedFramebufferRenderbuffer =
      unsafe { transmute(loader("glNamedFramebufferRenderbuffer\0".as_ptr())) };
    self.glNamedFramebufferTexture =
      unsafe { transmute(loader("glNamedFramebufferTexture\0".as_ptr())) };
    self.glNamedFramebufferTextureLayer =
      unsafe { transmute(loader("glNamedFramebufferTextureLayer\0".as_ptr())) };
    self.glNamedRenderbufferStorage =
      unsafe { transmute(loader("glNamedRenderbufferStorage\0".as_ptr())) };
    self.glNamedRenderbufferStorageMultisample =
      unsafe { transmute(loader("glNamedRenderbufferStorageMultisample\0".as_ptr())) };
    self.glNormalP3ui = unsafe { transmute(loader("glNormalP3ui\0".as_ptr())) };
    self.glNormalP3uiv = unsafe { transmute(loader("glNormalP3uiv\0".as_ptr())) };
    self.glObjectLabel = unsafe { transmute(loader("glObjectLabel\0".as_ptr())) };
    self.glObjectLabelKHR = unsafe { transmute(loader("glObjectLabelKHR\0".as_ptr())) };
    self.glObjectPtrLabel = unsafe { transmute(loader("glObjectPtrLabel\0".as_ptr())) };
    self.glObjectPtrLabelKHR = unsafe { transmute(loader("glObjectPtrLabelKHR\0".as_ptr())) };
    self.glPatchParameterfv = unsafe { transmute(loader("glPatchParameterfv\0".as_ptr())) };
    self.glPatchParameteri = unsafe { transmute(loader("glPatchParameteri\0".as_ptr())) };
    self.glPauseTransformFeedback =
      unsafe { transmute(loader("glPauseTransformFeedback\0".as_ptr())) };
    self.glPixelStoref = unsafe { transmute(loader("glPixelStoref\0".as_ptr())) };
    self.glPixelStorei = unsafe { transmute(loader("glPixelStorei\0".as_ptr())) };
    self.glPointParameterf = unsafe { transmute(loader("glPointParameterf\0".as_ptr())) };
    self.glPointParameterfv = unsafe { transmute(loader("glPointParameterfv\0".as_ptr())) };
    self.glPointParameteri = unsafe { transmute(loader("glPointParameteri\0".as_ptr())) };
    self.glPointParameteriv = unsafe { transmute(loader("glPointParameteriv\0".as_ptr())) };
    self.glPointSize = unsafe { transmute(loader("glPointSize\0".as_ptr())) };
    self.glPolygonMode = unsafe { transmute(loader("glPolygonMode\0".as_ptr())) };
    self.glPolygonOffset = unsafe { transmute(loader("glPolygonOffset\0".as_ptr())) };
    self.glPolygonOffsetClamp = unsafe { transmute(loader("glPolygonOffsetClamp\0".as_ptr())) };
    self.glPopDebugGroup = unsafe { transmute(loader("glPopDebugGroup\0".as_ptr())) };
    self.glPopDebugGroupKHR = unsafe { transmute(loader("glPopDebugGroupKHR\0".as_ptr())) };
    self.glPrimitiveBoundingBox = unsafe { transmute(loader("glPrimitiveBoundingBox\0".as_ptr())) };
    self.glPrimitiveRestartIndex =
      unsafe { transmute(loader("glPrimitiveRestartIndex\0".as_ptr())) };
    self.glProgramBinary = unsafe { transmute(loader("glProgramBinary\0".as_ptr())) };
    self.glProgramParameteri = unsafe { transmute(loader("glProgramParameteri\0".as_ptr())) };
    self.glProgramUniform1d = unsafe { transmute(loader("glProgramUniform1d\0".as_ptr())) };
    self.glProgramUniform1dv = unsafe { transmute(loader("glProgramUniform1dv\0".as_ptr())) };
    self.glProgramUniform1f = unsafe { transmute(loader("glProgramUniform1f\0".as_ptr())) };
    self.glProgramUniform1fv = unsafe { transmute(loader("glProgramUniform1fv\0".as_ptr())) };
    self.glProgramUniform1i = unsafe { transmute(loader("glProgramUniform1i\0".as_ptr())) };
    self.glProgramUniform1iv = unsafe { transmute(loader("glProgramUniform1iv\0".as_ptr())) };
    self.glProgramUniform1ui = unsafe { transmute(loader("glProgramUniform1ui\0".as_ptr())) };
    self.glProgramUniform1uiv = unsafe { transmute(loader("glProgramUniform1uiv\0".as_ptr())) };
    self.glProgramUniform2d = unsafe { transmute(loader("glProgramUniform2d\0".as_ptr())) };
    self.glProgramUniform2dv = unsafe { transmute(loader("glProgramUniform2dv\0".as_ptr())) };
    self.glProgramUniform2f = unsafe { transmute(loader("glProgramUniform2f\0".as_ptr())) };
    self.glProgramUniform2fv = unsafe { transmute(loader("glProgramUniform2fv\0".as_ptr())) };
    self.glProgramUniform2i = unsafe { transmute(loader("glProgramUniform2i\0".as_ptr())) };
    self.glProgramUniform2iv = unsafe { transmute(loader("glProgramUniform2iv\0".as_ptr())) };
    self.glProgramUniform2ui = unsafe { transmute(loader("glProgramUniform2ui\0".as_ptr())) };
    self.glProgramUniform2uiv = unsafe { transmute(loader("glProgramUniform2uiv\0".as_ptr())) };
    self.glProgramUniform3d = unsafe { transmute(loader("glProgramUniform3d\0".as_ptr())) };
    self.glProgramUniform3dv = unsafe { transmute(loader("glProgramUniform3dv\0".as_ptr())) };
    self.glProgramUniform3f = unsafe { transmute(loader("glProgramUniform3f\0".as_ptr())) };
    self.glProgramUniform3fv = unsafe { transmute(loader("glProgramUniform3fv\0".as_ptr())) };
    self.glProgramUniform3i = unsafe { transmute(loader("glProgramUniform3i\0".as_ptr())) };
    self.glProgramUniform3iv = unsafe { transmute(loader("glProgramUniform3iv\0".as_ptr())) };
    self.glProgramUniform3ui = unsafe { transmute(loader("glProgramUniform3ui\0".as_ptr())) };
    self.glProgramUniform3uiv = unsafe { transmute(loader("glProgramUniform3uiv\0".as_ptr())) };
    self.glProgramUniform4d = unsafe { transmute(loader("glProgramUniform4d\0".as_ptr())) };
    self.glProgramUniform4dv = unsafe { transmute(loader("glProgramUniform4dv\0".as_ptr())) };
    self.glProgramUniform4f = unsafe { transmute(loader("glProgramUniform4f\0".as_ptr())) };
    self.glProgramUniform4fv = unsafe { transmute(loader("glProgramUniform4fv\0".as_ptr())) };
    self.glProgramUniform4i = unsafe { transmute(loader("glProgramUniform4i\0".as_ptr())) };
    self.glProgramUniform4iv = unsafe { transmute(loader("glProgramUniform4iv\0".as_ptr())) };
    self.glProgramUniform4ui = unsafe { transmute(loader("glProgramUniform4ui\0".as_ptr())) };
    self.glProgramUniform4uiv = unsafe { transmute(loader("glProgramUniform4uiv\0".as_ptr())) };
    self.glProgramUniformMatrix2dv =
      unsafe { transmute(loader("glProgramUniformMatrix2dv\0".as_ptr())) };
    self.glProgramUniformMatrix2fv =
      unsafe { transmute(loader("glProgramUniformMatrix2fv\0".as_ptr())) };
    self.glProgramUniformMatrix2x3dv =
      unsafe { transmute(loader("glProgramUniformMatrix2x3dv\0".as_ptr())) };
    self.glProgramUniformMatrix2x3fv =
      unsafe { transmute(loader("glProgramUniformMatrix2x3fv\0".as_ptr())) };
    self.glProgramUniformMatrix2x4dv =
      unsafe { transmute(loader("glProgramUniformMatrix2x4dv\0".as_ptr())) };
    self.glProgramUniformMatrix2x4fv =
      unsafe { transmute(loader("glProgramUniformMatrix2x4fv\0".as_ptr())) };
    self.glProgramUniformMatrix3dv =
      unsafe { transmute(loader("glProgramUniformMatrix3dv\0".as_ptr())) };
    self.glProgramUniformMatrix3fv =
      unsafe { transmute(loader("glProgramUniformMatrix3fv\0".as_ptr())) };
    self.glProgramUniformMatrix3x2dv =
      unsafe { transmute(loader("glProgramUniformMatrix3x2dv\0".as_ptr())) };
    self.glProgramUniformMatrix3x2fv =
      unsafe { transmute(loader("glProgramUniformMatrix3x2fv\0".as_ptr())) };
    self.glProgramUniformMatrix3x4dv =
      unsafe { transmute(loader("glProgramUniformMatrix3x4dv\0".as_ptr())) };
    self.glProgramUniformMatrix3x4fv =
      unsafe { transmute(loader("glProgramUniformMatrix3x4fv\0".as_ptr())) };
    self.glProgramUniformMatrix4dv =
      unsafe { transmute(loader("glProgramUniformMatrix4dv\0".as_ptr())) };
    self.glProgramUniformMatrix4fv =
      unsafe { transmute(loader("glProgramUniformMatrix4fv\0".as_ptr())) };
    self.glProgramUniformMatrix4x2dv =
      unsafe { transmute(loader("glProgramUniformMatrix4x2dv\0".as_ptr())) };
    self.glProgramUniformMatrix4x2fv =
      unsafe { transmute(loader("glProgramUniformMatrix4x2fv\0".as_ptr())) };
    self.glProgramUniformMatrix4x3dv =
      unsafe { transmute(loader("glProgramUniformMatrix4x3dv\0".as_ptr())) };
    self.glProgramUniformMatrix4x3fv =
      unsafe { transmute(loader("glProgramUniformMatrix4x3fv\0".as_ptr())) };
    self.glProvokingVertex = unsafe { transmute(loader("glProvokingVertex\0".as_ptr())) };
    self.glPushDebugGroup = unsafe { transmute(loader("glPushDebugGroup\0".as_ptr())) };
    self.glPushDebugGroupKHR = unsafe { transmute(loader("glPushDebugGroupKHR\0".as_ptr())) };
    self.glQueryCounter = unsafe { transmute(loader("glQueryCounter\0".as_ptr())) };
    self.glReadBuffer = unsafe { transmute(loader("glReadBuffer\0".as_ptr())) };
    self.glReadPixels = unsafe { transmute(loader("glReadPixels\0".as_ptr())) };
    self.glReadnPixels = unsafe { transmute(loader("glReadnPixels\0".as_ptr())) };
    self.glReleaseShaderCompiler =
      unsafe { transmute(loader("glReleaseShaderCompiler\0".as_ptr())) };
    self.glRenderbufferStorage = unsafe { transmute(loader("glRenderbufferStorage\0".as_ptr())) };
    self.glRenderbufferStorageMultisample =
      unsafe { transmute(loader("glRenderbufferStorageMultisample\0".as_ptr())) };
    self.glResumeTransformFeedback =
      unsafe { transmute(loader("glResumeTransformFeedback\0".as_ptr())) };
    self.glSampleCoverage = unsafe { transmute(loader("glSampleCoverage\0".as_ptr())) };
    self.glSampleMaski = unsafe { transmute(loader("glSampleMaski\0".as_ptr())) };
    self.glSamplerParameterIiv = unsafe { transmute(loader("glSamplerParameterIiv\0".as_ptr())) };
    self.glSamplerParameterIuiv = unsafe { transmute(loader("glSamplerParameterIuiv\0".as_ptr())) };
    self.glSamplerParameterf = unsafe { transmute(loader("glSamplerParameterf\0".as_ptr())) };
    self.glSamplerParameterfv = unsafe { transmute(loader("glSamplerParameterfv\0".as_ptr())) };
    self.glSamplerParameteri = unsafe { transmute(loader("glSamplerParameteri\0".as_ptr())) };
    self.glSamplerParameteriv = unsafe { transmute(loader("glSamplerParameteriv\0".as_ptr())) };
    self.glScissor = unsafe { transmute(loader("glScissor\0".as_ptr())) };
    self.glScissorArrayv = unsafe { transmute(loader("glScissorArrayv\0".as_ptr())) };
    self.glScissorIndexed = unsafe { transmute(loader("glScissorIndexed\0".as_ptr())) };
    self.glScissorIndexedv = unsafe { transmute(loader("glScissorIndexedv\0".as_ptr())) };
    self.glSecondaryColorP3ui = unsafe { transmute(loader("glSecondaryColorP3ui\0".as_ptr())) };
    self.glSecondaryColorP3uiv = unsafe { transmute(loader("glSecondaryColorP3uiv\0".as_ptr())) };
    self.glShaderBinary = unsafe { transmute(loader("glShaderBinary\0".as_ptr())) };
    self.glShaderSource = unsafe { transmute(loader("glShaderSource\0".as_ptr())) };
    self.glShaderStorageBlockBinding =
      unsafe { transmute(loader("glShaderStorageBlockBinding\0".as_ptr())) };
    self.glSpecializeShader = unsafe { transmute(loader("glSpecializeShader\0".as_ptr())) };
    self.glStencilFunc = unsafe { transmute(loader("glStencilFunc\0".as_ptr())) };
    self.glStencilFuncSeparate = unsafe { transmute(loader("glStencilFuncSeparate\0".as_ptr())) };
    self.glStencilMask = unsafe { transmute(loader("glStencilMask\0".as_ptr())) };
    self.glStencilMaskSeparate = unsafe { transmute(loader("glStencilMaskSeparate\0".as_ptr())) };
    self.glStencilOp = unsafe { transmute(loader("glStencilOp\0".as_ptr())) };
    self.glStencilOpSeparate = unsafe { transmute(loader("glStencilOpSeparate\0".as_ptr())) };
    self.glTexBuffer = unsafe { transmute(loader("glTexBuffer\0".as_ptr())) };
    self.glTexBufferRange = unsafe { transmute(loader("glTexBufferRange\0".as_ptr())) };
    self.glTexCoordP1ui = unsafe { transmute(loader("glTexCoordP1ui\0".as_ptr())) };
    self.glTexCoordP1uiv = unsafe { transmute(loader("glTexCoordP1uiv\0".as_ptr())) };
    self.glTexCoordP2ui = unsafe { transmute(loader("glTexCoordP2ui\0".as_ptr())) };
    self.glTexCoordP2uiv = unsafe { transmute(loader("glTexCoordP2uiv\0".as_ptr())) };
    self.glTexCoordP3ui = unsafe { transmute(loader("glTexCoordP3ui\0".as_ptr())) };
    self.glTexCoordP3uiv = unsafe { transmute(loader("glTexCoordP3uiv\0".as_ptr())) };
    self.glTexCoordP4ui = unsafe { transmute(loader("glTexCoordP4ui\0".as_ptr())) };
    self.glTexCoordP4uiv = unsafe { transmute(loader("glTexCoordP4uiv\0".as_ptr())) };
    self.glTexImage1D = unsafe { transmute(loader("glTexImage1D\0".as_ptr())) };
    self.glTexImage2D = unsafe { transmute(loader("glTexImage2D\0".as_ptr())) };
    self.glTexImage2DMultisample =
      unsafe { transmute(loader("glTexImage2DMultisample\0".as_ptr())) };
    self.glTexImage3D = unsafe { transmute(loader("glTexImage3D\0".as_ptr())) };
    self.glTexImage3DMultisample =
      unsafe { transmute(loader("glTexImage3DMultisample\0".as_ptr())) };
    self.glTexParameterIiv = unsafe { transmute(loader("glTexParameterIiv\0".as_ptr())) };
    self.glTexParameterIuiv = unsafe { transmute(loader("glTexParameterIuiv\0".as_ptr())) };
    self.glTexParameterf = unsafe { transmute(loader("glTexParameterf\0".as_ptr())) };
    self.glTexParameterfv = unsafe { transmute(loader("glTexParameterfv\0".as_ptr())) };
    self.glTexParameteri = unsafe { transmute(loader("glTexParameteri\0".as_ptr())) };
    self.glTexParameteriv = unsafe { transmute(loader("glTexParameteriv\0".as_ptr())) };
    self.glTexStorage1D = unsafe { transmute(loader("glTexStorage1D\0".as_ptr())) };
    self.glTexStorage2D = unsafe { transmute(loader("glTexStorage2D\0".as_ptr())) };
    self.glTexStorage2DMultisample =
      unsafe { transmute(loader("glTexStorage2DMultisample\0".as_ptr())) };
    self.glTexStorage3D = unsafe { transmute(loader("glTexStorage3D\0".as_ptr())) };
    self.glTexStorage3DMultisample =
      unsafe { transmute(loader("glTexStorage3DMultisample\0".as_ptr())) };
    self.glTexSubImage1D = unsafe { transmute(loader("glTexSubImage1D\0".as_ptr())) };
    self.glTexSubImage2D = unsafe { transmute(loader("glTexSubImage2D\0".as_ptr())) };
    self.glTexSubImage3D = unsafe { transmute(loader("glTexSubImage3D\0".as_ptr())) };
    self.glTextureBarrier = unsafe { transmute(loader("glTextureBarrier\0".as_ptr())) };
    self.glTextureBuffer = unsafe { transmute(loader("glTextureBuffer\0".as_ptr())) };
    self.glTextureBufferRange = unsafe { transmute(loader("glTextureBufferRange\0".as_ptr())) };
    self.glTextureParameterIiv = unsafe { transmute(loader("glTextureParameterIiv\0".as_ptr())) };
    self.glTextureParameterIuiv = unsafe { transmute(loader("glTextureParameterIuiv\0".as_ptr())) };
    self.glTextureParameterf = unsafe { transmute(loader("glTextureParameterf\0".as_ptr())) };
    self.glTextureParameterfv = unsafe { transmute(loader("glTextureParameterfv\0".as_ptr())) };
    self.glTextureParameteri = unsafe { transmute(loader("glTextureParameteri\0".as_ptr())) };
    self.glTextureParameteriv = unsafe { transmute(loader("glTextureParameteriv\0".as_ptr())) };
    self.glTextureStorage1D = unsafe { transmute(loader("glTextureStorage1D\0".as_ptr())) };
    self.glTextureStorage2D = unsafe { transmute(loader("glTextureStorage2D\0".as_ptr())) };
    self.glTextureStorage2DMultisample =
      unsafe { transmute(loader("glTextureStorage2DMultisample\0".as_ptr())) };
    self.glTextureStorage3D = unsafe { transmute(loader("glTextureStorage3D\0".as_ptr())) };
    self.glTextureStorage3DMultisample =
      unsafe { transmute(loader("glTextureStorage3DMultisample\0".as_ptr())) };
    self.glTextureSubImage1D = unsafe { transmute(loader("glTextureSubImage1D\0".as_ptr())) };
    self.glTextureSubImage2D = unsafe { transmute(loader("glTextureSubImage2D\0".as_ptr())) };
    self.glTextureSubImage3D = unsafe { transmute(loader("glTextureSubImage3D\0".as_ptr())) };
    self.glTextureView = unsafe { transmute(loader("glTextureView\0".as_ptr())) };
    self.glTransformFeedbackBufferBase =
      unsafe { transmute(loader("glTransformFeedbackBufferBase\0".as_ptr())) };
    self.glTransformFeedbackBufferRange =
      unsafe { transmute(loader("glTransformFeedbackBufferRange\0".as_ptr())) };
    self.glTransformFeedbackVaryings =
      unsafe { transmute(loader("glTransformFeedbackVaryings\0".as_ptr())) };
    self.glUniform1d = unsafe { transmute(loader("glUniform1d\0".as_ptr())) };
    self.glUniform1dv = unsafe { transmute(loader("glUniform1dv\0".as_ptr())) };
    self.glUniform1f = unsafe { transmute(loader("glUniform1f\0".as_ptr())) };
    self.glUniform1fv = unsafe { transmute(loader("glUniform1fv\0".as_ptr())) };
    self.glUniform1i = unsafe { transmute(loader("glUniform1i\0".as_ptr())) };
    self.glUniform1iv = unsafe { transmute(loader("glUniform1iv\0".as_ptr())) };
    self.glUniform1ui = unsafe { transmute(loader("glUniform1ui\0".as_ptr())) };
    self.glUniform1uiv = unsafe { transmute(loader("glUniform1uiv\0".as_ptr())) };
    self.glUniform2d = unsafe { transmute(loader("glUniform2d\0".as_ptr())) };
    self.glUniform2dv = unsafe { transmute(loader("glUniform2dv\0".as_ptr())) };
    self.glUniform2f = unsafe { transmute(loader("glUniform2f\0".as_ptr())) };
    self.glUniform2fv = unsafe { transmute(loader("glUniform2fv\0".as_ptr())) };
    self.glUniform2i = unsafe { transmute(loader("glUniform2i\0".as_ptr())) };
    self.glUniform2iv = unsafe { transmute(loader("glUniform2iv\0".as_ptr())) };
    self.glUniform2ui = unsafe { transmute(loader("glUniform2ui\0".as_ptr())) };
    self.glUniform2uiv = unsafe { transmute(loader("glUniform2uiv\0".as_ptr())) };
    self.glUniform3d = unsafe { transmute(loader("glUniform3d\0".as_ptr())) };
    self.glUniform3dv = unsafe { transmute(loader("glUniform3dv\0".as_ptr())) };
    self.glUniform3f = unsafe { transmute(loader("glUniform3f\0".as_ptr())) };
    self.glUniform3fv = unsafe { transmute(loader("glUniform3fv\0".as_ptr())) };
    self.glUniform3i = unsafe { transmute(loader("glUniform3i\0".as_ptr())) };
    self.glUniform3iv = unsafe { transmute(loader("glUniform3iv\0".as_ptr())) };
    self.glUniform3ui = unsafe { transmute(loader("glUniform3ui\0".as_ptr())) };
    self.glUniform3uiv = unsafe { transmute(loader("glUniform3uiv\0".as_ptr())) };
    self.glUniform4d = unsafe { transmute(loader("glUniform4d\0".as_ptr())) };
    self.glUniform4dv = unsafe { transmute(loader("glUniform4dv\0".as_ptr())) };
    self.glUniform4f = unsafe { transmute(loader("glUniform4f\0".as_ptr())) };
    self.glUniform4fv = unsafe { transmute(loader("glUniform4fv\0".as_ptr())) };
    self.glUniform4i = unsafe { transmute(loader("glUniform4i\0".as_ptr())) };
    self.glUniform4iv = unsafe { transmute(loader("glUniform4iv\0".as_ptr())) };
    self.glUniform4ui = unsafe { transmute(loader("glUniform4ui\0".as_ptr())) };
    self.glUniform4uiv = unsafe { transmute(loader("glUniform4uiv\0".as_ptr())) };
    self.glUniformBlockBinding = unsafe { transmute(loader("glUniformBlockBinding\0".as_ptr())) };
    self.glUniformMatrix2dv = unsafe { transmute(loader("glUniformMatrix2dv\0".as_ptr())) };
    self.glUniformMatrix2fv = unsafe { transmute(loader("glUniformMatrix2fv\0".as_ptr())) };
    self.glUniformMatrix2x3dv = unsafe { transmute(loader("glUniformMatrix2x3dv\0".as_ptr())) };
    self.glUniformMatrix2x3fv = unsafe { transmute(loader("glUniformMatrix2x3fv\0".as_ptr())) };
    self.glUniformMatrix2x4dv = unsafe { transmute(loader("glUniformMatrix2x4dv\0".as_ptr())) };
    self.glUniformMatrix2x4fv = unsafe { transmute(loader("glUniformMatrix2x4fv\0".as_ptr())) };
    self.glUniformMatrix3dv = unsafe { transmute(loader("glUniformMatrix3dv\0".as_ptr())) };
    self.glUniformMatrix3fv = unsafe { transmute(loader("glUniformMatrix3fv\0".as_ptr())) };
    self.glUniformMatrix3x2dv = unsafe { transmute(loader("glUniformMatrix3x2dv\0".as_ptr())) };
    self.glUniformMatrix3x2fv = unsafe { transmute(loader("glUniformMatrix3x2fv\0".as_ptr())) };
    self.glUniformMatrix3x4dv = unsafe { transmute(loader("glUniformMatrix3x4dv\0".as_ptr())) };
    self.glUniformMatrix3x4fv = unsafe { transmute(loader("glUniformMatrix3x4fv\0".as_ptr())) };
    self.glUniformMatrix4dv = unsafe { transmute(loader("glUniformMatrix4dv\0".as_ptr())) };
    self.glUniformMatrix4fv = unsafe { transmute(loader("glUniformMatrix4fv\0".as_ptr())) };
    self.glUniformMatrix4x2dv = unsafe { transmute(loader("glUniformMatrix4x2dv\0".as_ptr())) };
    self.glUniformMatrix4x2fv = unsafe { transmute(loader("glUniformMatrix4x2fv\0".as_ptr())) };
    self.glUniformMatrix4x3dv = unsafe { transmute(loader("glUniformMatrix4x3dv\0".as_ptr())) };
    self.glUniformMatrix4x3fv = unsafe { transmute(loader("glUniformMatrix4x3fv\0".as_ptr())) };
    self.glUniformSubroutinesuiv =
      unsafe { transmute(loader("glUniformSubroutinesuiv\0".as_ptr())) };
    self.glUnmapBuffer = unsafe { transmute(loader("glUnmapBuffer\0".as_ptr())) };
    self.glUnmapNamedBuffer = unsafe { transmute(loader("glUnmapNamedBuffer\0".as_ptr())) };
    self.glUseProgram = unsafe { transmute(loader("glUseProgram\0".as_ptr())) };
    self.glUseProgramStages = unsafe { transmute(loader("glUseProgramStages\0".as_ptr())) };
    self.glValidateProgram = unsafe { transmute(loader("glValidateProgram\0".as_ptr())) };
    self.glValidateProgramPipeline =
      unsafe { transmute(loader("glValidateProgramPipeline\0".as_ptr())) };
    self.glVertexArrayAttribBinding =
      unsafe { transmute(loader("glVertexArrayAttribBinding\0".as_ptr())) };
    self.glVertexArrayAttribFormat =
      unsafe { transmute(loader("glVertexArrayAttribFormat\0".as_ptr())) };
    self.glVertexArrayAttribIFormat =
      unsafe { transmute(loader("glVertexArrayAttribIFormat\0".as_ptr())) };
    self.glVertexArrayAttribLFormat =
      unsafe { transmute(loader("glVertexArrayAttribLFormat\0".as_ptr())) };
    self.glVertexArrayBindingDivisor =
      unsafe { transmute(loader("glVertexArrayBindingDivisor\0".as_ptr())) };
    self.glVertexArrayElementBuffer =
      unsafe { transmute(loader("glVertexArrayElementBuffer\0".as_ptr())) };
    self.glVertexArrayVertexBuffer =
      unsafe { transmute(loader("glVertexArrayVertexBuffer\0".as_ptr())) };
    self.glVertexArrayVertexBuffers =
      unsafe { transmute(loader("glVertexArrayVertexBuffers\0".as_ptr())) };
    self.glVertexAttrib1d = unsafe { transmute(loader("glVertexAttrib1d\0".as_ptr())) };
    self.glVertexAttrib1dv = unsafe { transmute(loader("glVertexAttrib1dv\0".as_ptr())) };
    self.glVertexAttrib1f = unsafe { transmute(loader("glVertexAttrib1f\0".as_ptr())) };
    self.glVertexAttrib1fv = unsafe { transmute(loader("glVertexAttrib1fv\0".as_ptr())) };
    self.glVertexAttrib1s = unsafe { transmute(loader("glVertexAttrib1s\0".as_ptr())) };
    self.glVertexAttrib1sv = unsafe { transmute(loader("glVertexAttrib1sv\0".as_ptr())) };
    self.glVertexAttrib2d = unsafe { transmute(loader("glVertexAttrib2d\0".as_ptr())) };
    self.glVertexAttrib2dv = unsafe { transmute(loader("glVertexAttrib2dv\0".as_ptr())) };
    self.glVertexAttrib2f = unsafe { transmute(loader("glVertexAttrib2f\0".as_ptr())) };
    self.glVertexAttrib2fv = unsafe { transmute(loader("glVertexAttrib2fv\0".as_ptr())) };
    self.glVertexAttrib2s = unsafe { transmute(loader("glVertexAttrib2s\0".as_ptr())) };
    self.glVertexAttrib2sv = unsafe { transmute(loader("glVertexAttrib2sv\0".as_ptr())) };
    self.glVertexAttrib3d = unsafe { transmute(loader("glVertexAttrib3d\0".as_ptr())) };
    self.glVertexAttrib3dv = unsafe { transmute(loader("glVertexAttrib3dv\0".as_ptr())) };
    self.glVertexAttrib3f = unsafe { transmute(loader("glVertexAttrib3f\0".as_ptr())) };
    self.glVertexAttrib3fv = unsafe { transmute(loader("glVertexAttrib3fv\0".as_ptr())) };
    self.glVertexAttrib3s = unsafe { transmute(loader("glVertexAttrib3s\0".as_ptr())) };
    self.glVertexAttrib3sv = unsafe { transmute(loader("glVertexAttrib3sv\0".as_ptr())) };
    self.glVertexAttrib4Nbv = unsafe { transmute(loader("glVertexAttrib4Nbv\0".as_ptr())) };
    self.glVertexAttrib4Niv = unsafe { transmute(loader("glVertexAttrib4Niv\0".as_ptr())) };
    self.glVertexAttrib4Nsv = unsafe { transmute(loader("glVertexAttrib4Nsv\0".as_ptr())) };
    self.glVertexAttrib4Nub = unsafe { transmute(loader("glVertexAttrib4Nub\0".as_ptr())) };
    self.glVertexAttrib4Nubv = unsafe { transmute(loader("glVertexAttrib4Nubv\0".as_ptr())) };
    self.glVertexAttrib4Nuiv = unsafe { transmute(loader("glVertexAttrib4Nuiv\0".as_ptr())) };
    self.glVertexAttrib4Nusv = unsafe { transmute(loader("glVertexAttrib4Nusv\0".as_ptr())) };
    self.glVertexAttrib4bv = unsafe { transmute(loader("glVertexAttrib4bv\0".as_ptr())) };
    self.glVertexAttrib4d = unsafe { transmute(loader("glVertexAttrib4d\0".as_ptr())) };
    self.glVertexAttrib4dv = unsafe { transmute(loader("glVertexAttrib4dv\0".as_ptr())) };
    self.glVertexAttrib4f = unsafe { transmute(loader("glVertexAttrib4f\0".as_ptr())) };
    self.glVertexAttrib4fv = unsafe { transmute(loader("glVertexAttrib4fv\0".as_ptr())) };
    self.glVertexAttrib4iv = unsafe { transmute(loader("glVertexAttrib4iv\0".as_ptr())) };
    self.glVertexAttrib4s = unsafe { transmute(loader("glVertexAttrib4s\0".as_ptr())) };
    self.glVertexAttrib4sv = unsafe { transmute(loader("glVertexAttrib4sv\0".as_ptr())) };
    self.glVertexAttrib4ubv = unsafe { transmute(loader("glVertexAttrib4ubv\0".as_ptr())) };
    self.glVertexAttrib4uiv = unsafe { transmute(loader("glVertexAttrib4uiv\0".as_ptr())) };
    self.glVertexAttrib4usv = unsafe { transmute(loader("glVertexAttrib4usv\0".as_ptr())) };
    self.glVertexAttribBinding = unsafe { transmute(loader("glVertexAttribBinding\0".as_ptr())) };
    self.glVertexAttribDivisor = unsafe { transmute(loader("glVertexAttribDivisor\0".as_ptr())) };
    self.glVertexAttribFormat = unsafe { transmute(loader("glVertexAttribFormat\0".as_ptr())) };
    self.glVertexAttribI1i = unsafe { transmute(loader("glVertexAttribI1i\0".as_ptr())) };
    self.glVertexAttribI1iv = unsafe { transmute(loader("glVertexAttribI1iv\0".as_ptr())) };
    self.glVertexAttribI1ui = unsafe { transmute(loader("glVertexAttribI1ui\0".as_ptr())) };
    self.glVertexAttribI1uiv = unsafe { transmute(loader("glVertexAttribI1uiv\0".as_ptr())) };
    self.glVertexAttribI2i = unsafe { transmute(loader("glVertexAttribI2i\0".as_ptr())) };
    self.glVertexAttribI2iv = unsafe { transmute(loader("glVertexAttribI2iv\0".as_ptr())) };
    self.glVertexAttribI2ui = unsafe { transmute(loader("glVertexAttribI2ui\0".as_ptr())) };
    self.glVertexAttribI2uiv = unsafe { transmute(loader("glVertexAttribI2uiv\0".as_ptr())) };
    self.glVertexAttribI3i = unsafe { transmute(loader("glVertexAttribI3i\0".as_ptr())) };
    self.glVertexAttribI3iv = unsafe { transmute(loader("glVertexAttribI3iv\0".as_ptr())) };
    self.glVertexAttribI3ui = unsafe { transmute(loader("glVertexAttribI3ui\0".as_ptr())) };
    self.glVertexAttribI3uiv = unsafe { transmute(loader("glVertexAttribI3uiv\0".as_ptr())) };
    self.glVertexAttribI4bv = unsafe { transmute(loader("glVertexAttribI4bv\0".as_ptr())) };
    self.glVertexAttribI4i = unsafe { transmute(loader("glVertexAttribI4i\0".as_ptr())) };
    self.glVertexAttribI4iv = unsafe { transmute(loader("glVertexAttribI4iv\0".as_ptr())) };
    self.glVertexAttribI4sv = unsafe { transmute(loader("glVertexAttribI4sv\0".as_ptr())) };
    self.glVertexAttribI4ubv = unsafe { transmute(loader("glVertexAttribI4ubv\0".as_ptr())) };
    self.glVertexAttribI4ui = unsafe { transmute(loader("glVertexAttribI4ui\0".as_ptr())) };
    self.glVertexAttribI4uiv = unsafe { transmute(loader("glVertexAttribI4uiv\0".as_ptr())) };
    self.glVertexAttribI4usv = unsafe { transmute(loader("glVertexAttribI4usv\0".as_ptr())) };
    self.glVertexAttribIFormat = unsafe { transmute(loader("glVertexAttribIFormat\0".as_ptr())) };
    self.glVertexAttribIPointer = unsafe { transmute(loader("glVertexAttribIPointer\0".as_ptr())) };
    self.glVertexAttribL1d = unsafe { transmute(loader("glVertexAttribL1d\0".as_ptr())) };
    self.glVertexAttribL1dv = unsafe { transmute(loader("glVertexAttribL1dv\0".as_ptr())) };
    self.glVertexAttribL2d = unsafe { transmute(loader("glVertexAttribL2d\0".as_ptr())) };
    self.glVertexAttribL2dv = unsafe { transmute(loader("glVertexAttribL2dv\0".as_ptr())) };
    self.glVertexAttribL3d = unsafe { transmute(loader("glVertexAttribL3d\0".as_ptr())) };
    self.glVertexAttribL3dv = unsafe { transmute(loader("glVertexAttribL3dv\0".as_ptr())) };
    self.glVertexAttribL4d = unsafe { transmute(loader("glVertexAttribL4d\0".as_ptr())) };
    self.glVertexAttribL4dv = unsafe { transmute(loader("glVertexAttribL4dv\0".as_ptr())) };
    self.glVertexAttribLFormat = unsafe { transmute(loader("glVertexAttribLFormat\0".as_ptr())) };
    self.glVertexAttribLPointer = unsafe { transmute(loader("glVertexAttribLPointer\0".as_ptr())) };
    self.glVertexAttribP1ui = unsafe { transmute(loader("glVertexAttribP1ui\0".as_ptr())) };
    self.glVertexAttribP1uiv = unsafe { transmute(loader("glVertexAttribP1uiv\0".as_ptr())) };
    self.glVertexAttribP2ui = unsafe { transmute(loader("glVertexAttribP2ui\0".as_ptr())) };
    self.glVertexAttribP2uiv = unsafe { transmute(loader("glVertexAttribP2uiv\0".as_ptr())) };
    self.glVertexAttribP3ui = unsafe { transmute(loader("glVertexAttribP3ui\0".as_ptr())) };
    self.glVertexAttribP3uiv = unsafe { transmute(loader("glVertexAttribP3uiv\0".as_ptr())) };
    self.glVertexAttribP4ui = unsafe { transmute(loader("glVertexAttribP4ui\0".as_ptr())) };
    self.glVertexAttribP4uiv = unsafe { transmute(loader("glVertexAttribP4uiv\0".as_ptr())) };
    self.glVertexAttribPointer = unsafe { transmute(loader("glVertexAttribPointer\0".as_ptr())) };
    self.glVertexBindingDivisor = unsafe { transmute(loader("glVertexBindingDivisor\0".as_ptr())) };
    self.glVertexP2ui = unsafe { transmute(loader("glVertexP2ui\0".as_ptr())) };
    self.glVertexP2uiv = unsafe { transmute(loader("glVertexP2uiv\0".as_ptr())) };
    self.glVertexP3ui = unsafe { transmute(loader("glVertexP3ui\0".as_ptr())) };
    self.glVertexP3uiv = unsafe { transmute(loader("glVertexP3uiv\0".as_ptr())) };
    self.glVertexP4ui = unsafe { transmute(loader("glVertexP4ui\0".as_ptr())) };
    self.glVertexP4uiv = unsafe { transmute(loader("glVertexP4uiv\0".as_ptr())) };
    self.glViewport = unsafe { transmute(loader("glViewport\0".as_ptr())) };
    self.glViewportArrayv = unsafe { transmute(loader("glViewportArrayv\0".as_ptr())) };
    self.glViewportIndexedf = unsafe { transmute(loader("glViewportIndexedf\0".as_ptr())) };
    self.glViewportIndexedfv = unsafe { transmute(loader("glViewportIndexedfv\0".as_ptr())) };
    self.glWaitSync = unsafe { transmute(loader("glWaitSync\0".as_ptr())) };
  }
}

/// This 'literal' value might help if you wanna make a GlFns in a `static mut`
/// or something.
#[allow(unused)]
#[doc(hidden)]
pub const BLANK_GL_FNS: GlFns = GlFns {
  glActiveShaderProgram: None,
  glActiveTexture: None,
  glAttachShader: None,
  glBeginConditionalRender: None,
  glBeginQuery: None,
  glBeginQueryIndexed: None,
  glBeginTransformFeedback: None,
  glBindAttribLocation: None,
  glBindBuffer: None,
  glBindBufferBase: None,
  glBindBufferRange: None,
  glBindBuffersBase: None,
  glBindBuffersRange: None,
  glBindFragDataLocation: None,
  glBindFragDataLocationIndexed: None,
  glBindFramebuffer: None,
  glBindImageTexture: None,
  glBindImageTextures: None,
  glBindProgramPipeline: None,
  glBindRenderbuffer: None,
  glBindSampler: None,
  glBindSamplers: None,
  glBindTexture: None,
  glBindTextureUnit: None,
  glBindTextures: None,
  glBindTransformFeedback: None,
  glBindVertexArray: None,
  glBindVertexBuffer: None,
  glBindVertexBuffers: None,
  glBlendBarrier: None,
  glBlendColor: None,
  glBlendEquation: None,
  glBlendEquationSeparate: None,
  glBlendEquationSeparatei: None,
  glBlendEquationi: None,
  glBlendFunc: None,
  glBlendFuncSeparate: None,
  glBlendFuncSeparatei: None,
  glBlendFunci: None,
  glBlitFramebuffer: None,
  glBlitNamedFramebuffer: None,
  glBufferData: None,
  glBufferStorage: None,
  glBufferSubData: None,
  glCheckFramebufferStatus: None,
  glCheckNamedFramebufferStatus: None,
  glClampColor: None,
  glClear: None,
  glClearBufferData: None,
  glClearBufferSubData: None,
  glClearBufferfi: None,
  glClearBufferfv: None,
  glClearBufferiv: None,
  glClearBufferuiv: None,
  glClearColor: None,
  glClearDepth: None,
  glClearDepthf: None,
  glClearNamedBufferData: None,
  glClearNamedBufferSubData: None,
  glClearNamedFramebufferfi: None,
  glClearNamedFramebufferfv: None,
  glClearNamedFramebufferiv: None,
  glClearNamedFramebufferuiv: None,
  glClearStencil: None,
  glClearTexImage: None,
  glClearTexSubImage: None,
  glClientWaitSync: None,
  glClipControl: None,
  glColorMask: None,
  glColorMaski: None,
  glColorP3ui: None,
  glColorP3uiv: None,
  glColorP4ui: None,
  glColorP4uiv: None,
  glCompileShader: None,
  glCompressedTexImage1D: None,
  glCompressedTexImage2D: None,
  glCompressedTexImage3D: None,
  glCompressedTexSubImage1D: None,
  glCompressedTexSubImage2D: None,
  glCompressedTexSubImage3D: None,
  glCompressedTextureSubImage1D: None,
  glCompressedTextureSubImage2D: None,
  glCompressedTextureSubImage3D: None,
  glCopyBufferSubData: None,
  glCopyImageSubData: None,
  glCopyNamedBufferSubData: None,
  glCopyTexImage1D: None,
  glCopyTexImage2D: None,
  glCopyTexSubImage1D: None,
  glCopyTexSubImage2D: None,
  glCopyTexSubImage3D: None,
  glCopyTextureSubImage1D: None,
  glCopyTextureSubImage2D: None,
  glCopyTextureSubImage3D: None,
  glCreateBuffers: None,
  glCreateFramebuffers: None,
  glCreateProgram: None,
  glCreateProgramPipelines: None,
  glCreateQueries: None,
  glCreateRenderbuffers: None,
  glCreateSamplers: None,
  glCreateShader: None,
  glCreateShaderProgramv: None,
  glCreateTextures: None,
  glCreateTransformFeedbacks: None,
  glCreateVertexArrays: None,
  glCullFace: None,
  glDebugMessageCallback: None,
  glDebugMessageCallbackKHR: None,
  glDebugMessageControl: None,
  glDebugMessageControlKHR: None,
  glDebugMessageInsert: None,
  glDebugMessageInsertKHR: None,
  glDeleteBuffers: None,
  glDeleteFramebuffers: None,
  glDeleteProgram: None,
  glDeleteProgramPipelines: None,
  glDeleteQueries: None,
  glDeleteRenderbuffers: None,
  glDeleteSamplers: None,
  glDeleteShader: None,
  glDeleteSync: None,
  glDeleteTextures: None,
  glDeleteTransformFeedbacks: None,
  glDeleteVertexArrays: None,
  glDepthFunc: None,
  glDepthMask: None,
  glDepthRange: None,
  glDepthRangeArrayv: None,
  glDepthRangeIndexed: None,
  glDepthRangef: None,
  glDetachShader: None,
  glDisable: None,
  glDisableVertexArrayAttrib: None,
  glDisableVertexAttribArray: None,
  glDisablei: None,
  glDispatchCompute: None,
  glDispatchComputeIndirect: None,
  glDrawArrays: None,
  glDrawArraysIndirect: None,
  glDrawArraysInstanced: None,
  glDrawArraysInstancedBaseInstance: None,
  glDrawBuffer: None,
  glDrawBuffers: None,
  glDrawElements: None,
  glDrawElementsBaseVertex: None,
  glDrawElementsIndirect: None,
  glDrawElementsInstanced: None,
  glDrawElementsInstancedBaseInstance: None,
  glDrawElementsInstancedBaseVertex: None,
  glDrawElementsInstancedBaseVertexBaseInstance: None,
  glDrawRangeElements: None,
  glDrawRangeElementsBaseVertex: None,
  glDrawTransformFeedback: None,
  glDrawTransformFeedbackInstanced: None,
  glDrawTransformFeedbackStream: None,
  glDrawTransformFeedbackStreamInstanced: None,
  glEnable: None,
  glEnableVertexArrayAttrib: None,
  glEnableVertexAttribArray: None,
  glEnablei: None,
  glEndConditionalRender: None,
  glEndQuery: None,
  glEndQueryIndexed: None,
  glEndTransformFeedback: None,
  glFenceSync: None,
  glFinish: None,
  glFlush: None,
  glFlushMappedBufferRange: None,
  glFlushMappedNamedBufferRange: None,
  glFramebufferParameteri: None,
  glFramebufferRenderbuffer: None,
  glFramebufferTexture: None,
  glFramebufferTexture1D: None,
  glFramebufferTexture2D: None,
  glFramebufferTexture3D: None,
  glFramebufferTextureLayer: None,
  glFrontFace: None,
  glGenBuffers: None,
  glGenFramebuffers: None,
  glGenProgramPipelines: None,
  glGenQueries: None,
  glGenRenderbuffers: None,
  glGenSamplers: None,
  glGenTextures: None,
  glGenTransformFeedbacks: None,
  glGenVertexArrays: None,
  glGenerateMipmap: None,
  glGenerateTextureMipmap: None,
  glGetActiveAtomicCounterBufferiv: None,
  glGetActiveAttrib: None,
  glGetActiveSubroutineName: None,
  glGetActiveSubroutineUniformName: None,
  glGetActiveSubroutineUniformiv: None,
  glGetActiveUniform: None,
  glGetActiveUniformBlockName: None,
  glGetActiveUniformBlockiv: None,
  glGetActiveUniformName: None,
  glGetActiveUniformsiv: None,
  glGetAttachedShaders: None,
  glGetAttribLocation: None,
  glGetBooleani_v: None,
  glGetBooleanv: None,
  glGetBufferParameteri64v: None,
  glGetBufferParameteriv: None,
  glGetBufferPointerv: None,
  glGetBufferSubData: None,
  glGetCompressedTexImage: None,
  glGetCompressedTextureImage: None,
  glGetCompressedTextureSubImage: None,
  glGetDebugMessageLog: None,
  glGetDebugMessageLogKHR: None,
  glGetDoublei_v: None,
  glGetDoublev: None,
  glGetError: None,
  glGetFloati_v: None,
  glGetFloatv: None,
  glGetFragDataIndex: None,
  glGetFragDataLocation: None,
  glGetFramebufferAttachmentParameteriv: None,
  glGetFramebufferParameteriv: None,
  glGetGraphicsResetStatus: None,
  glGetInteger64i_v: None,
  glGetInteger64v: None,
  glGetIntegeri_v: None,
  glGetIntegerv: None,
  glGetInternalformati64v: None,
  glGetInternalformativ: None,
  glGetMultisamplefv: None,
  glGetNamedBufferParameteri64v: None,
  glGetNamedBufferParameteriv: None,
  glGetNamedBufferPointerv: None,
  glGetNamedBufferSubData: None,
  glGetNamedFramebufferAttachmentParameteriv: None,
  glGetNamedFramebufferParameteriv: None,
  glGetNamedRenderbufferParameteriv: None,
  glGetObjectLabel: None,
  glGetObjectLabelKHR: None,
  glGetObjectPtrLabel: None,
  glGetObjectPtrLabelKHR: None,
  glGetPointerv: None,
  glGetPointervKHR: None,
  glGetProgramBinary: None,
  glGetProgramInfoLog: None,
  glGetProgramInterfaceiv: None,
  glGetProgramPipelineInfoLog: None,
  glGetProgramPipelineiv: None,
  glGetProgramResourceIndex: None,
  glGetProgramResourceLocation: None,
  glGetProgramResourceLocationIndex: None,
  glGetProgramResourceName: None,
  glGetProgramResourceiv: None,
  glGetProgramStageiv: None,
  glGetProgramiv: None,
  glGetQueryBufferObjecti64v: None,
  glGetQueryBufferObjectiv: None,
  glGetQueryBufferObjectui64v: None,
  glGetQueryBufferObjectuiv: None,
  glGetQueryIndexediv: None,
  glGetQueryObjecti64v: None,
  glGetQueryObjectiv: None,
  glGetQueryObjectui64v: None,
  glGetQueryObjectuiv: None,
  glGetQueryiv: None,
  glGetRenderbufferParameteriv: None,
  glGetSamplerParameterIiv: None,
  glGetSamplerParameterIuiv: None,
  glGetSamplerParameterfv: None,
  glGetSamplerParameteriv: None,
  glGetShaderInfoLog: None,
  glGetShaderPrecisionFormat: None,
  glGetShaderSource: None,
  glGetShaderiv: None,
  glGetString: None,
  glGetStringi: None,
  glGetSubroutineIndex: None,
  glGetSubroutineUniformLocation: None,
  glGetSynciv: None,
  glGetTexImage: None,
  glGetTexLevelParameterfv: None,
  glGetTexLevelParameteriv: None,
  glGetTexParameterIiv: None,
  glGetTexParameterIuiv: None,
  glGetTexParameterfv: None,
  glGetTexParameteriv: None,
  glGetTextureImage: None,
  glGetTextureLevelParameterfv: None,
  glGetTextureLevelParameteriv: None,
  glGetTextureParameterIiv: None,
  glGetTextureParameterIuiv: None,
  glGetTextureParameterfv: None,
  glGetTextureParameteriv: None,
  glGetTextureSubImage: None,
  glGetTransformFeedbackVarying: None,
  glGetTransformFeedbacki64_v: None,
  glGetTransformFeedbacki_v: None,
  glGetTransformFeedbackiv: None,
  glGetUniformBlockIndex: None,
  glGetUniformIndices: None,
  glGetUniformLocation: None,
  glGetUniformSubroutineuiv: None,
  glGetUniformdv: None,
  glGetUniformfv: None,
  glGetUniformiv: None,
  glGetUniformuiv: None,
  glGetVertexArrayIndexed64iv: None,
  glGetVertexArrayIndexediv: None,
  glGetVertexArrayiv: None,
  glGetVertexAttribIiv: None,
  glGetVertexAttribIuiv: None,
  glGetVertexAttribLdv: None,
  glGetVertexAttribPointerv: None,
  glGetVertexAttribdv: None,
  glGetVertexAttribfv: None,
  glGetVertexAttribiv: None,
  glGetnColorTable: None,
  glGetnCompressedTexImage: None,
  glGetnConvolutionFilter: None,
  glGetnHistogram: None,
  glGetnMapdv: None,
  glGetnMapfv: None,
  glGetnMapiv: None,
  glGetnMinmax: None,
  glGetnPixelMapfv: None,
  glGetnPixelMapuiv: None,
  glGetnPixelMapusv: None,
  glGetnPolygonStipple: None,
  glGetnSeparableFilter: None,
  glGetnTexImage: None,
  glGetnUniformdv: None,
  glGetnUniformfv: None,
  glGetnUniformiv: None,
  glGetnUniformuiv: None,
  glHint: None,
  glInvalidateBufferData: None,
  glInvalidateBufferSubData: None,
  glInvalidateFramebuffer: None,
  glInvalidateNamedFramebufferData: None,
  glInvalidateNamedFramebufferSubData: None,
  glInvalidateSubFramebuffer: None,
  glInvalidateTexImage: None,
  glInvalidateTexSubImage: None,
  glIsBuffer: None,
  glIsEnabled: None,
  glIsEnabledi: None,
  glIsFramebuffer: None,
  glIsProgram: None,
  glIsProgramPipeline: None,
  glIsQuery: None,
  glIsRenderbuffer: None,
  glIsSampler: None,
  glIsShader: None,
  glIsSync: None,
  glIsTexture: None,
  glIsTransformFeedback: None,
  glIsVertexArray: None,
  glLineWidth: None,
  glLinkProgram: None,
  glLogicOp: None,
  glMapBuffer: None,
  glMapBufferRange: None,
  glMapNamedBuffer: None,
  glMapNamedBufferRange: None,
  glMemoryBarrier: None,
  glMemoryBarrierByRegion: None,
  glMinSampleShading: None,
  glMultiDrawArrays: None,
  glMultiDrawArraysIndirect: None,
  glMultiDrawArraysIndirectCount: None,
  glMultiDrawElements: None,
  glMultiDrawElementsBaseVertex: None,
  glMultiDrawElementsIndirect: None,
  glMultiDrawElementsIndirectCount: None,
  glMultiTexCoordP1ui: None,
  glMultiTexCoordP1uiv: None,
  glMultiTexCoordP2ui: None,
  glMultiTexCoordP2uiv: None,
  glMultiTexCoordP3ui: None,
  glMultiTexCoordP3uiv: None,
  glMultiTexCoordP4ui: None,
  glMultiTexCoordP4uiv: None,
  glNamedBufferData: None,
  glNamedBufferStorage: None,
  glNamedBufferSubData: None,
  glNamedFramebufferDrawBuffer: None,
  glNamedFramebufferDrawBuffers: None,
  glNamedFramebufferParameteri: None,
  glNamedFramebufferReadBuffer: None,
  glNamedFramebufferRenderbuffer: None,
  glNamedFramebufferTexture: None,
  glNamedFramebufferTextureLayer: None,
  glNamedRenderbufferStorage: None,
  glNamedRenderbufferStorageMultisample: None,
  glNormalP3ui: None,
  glNormalP3uiv: None,
  glObjectLabel: None,
  glObjectLabelKHR: None,
  glObjectPtrLabel: None,
  glObjectPtrLabelKHR: None,
  glPatchParameterfv: None,
  glPatchParameteri: None,
  glPauseTransformFeedback: None,
  glPixelStoref: None,
  glPixelStorei: None,
  glPointParameterf: None,
  glPointParameterfv: None,
  glPointParameteri: None,
  glPointParameteriv: None,
  glPointSize: None,
  glPolygonMode: None,
  glPolygonOffset: None,
  glPolygonOffsetClamp: None,
  glPopDebugGroup: None,
  glPopDebugGroupKHR: None,
  glPrimitiveBoundingBox: None,
  glPrimitiveRestartIndex: None,
  glProgramBinary: None,
  glProgramParameteri: None,
  glProgramUniform1d: None,
  glProgramUniform1dv: None,
  glProgramUniform1f: None,
  glProgramUniform1fv: None,
  glProgramUniform1i: None,
  glProgramUniform1iv: None,
  glProgramUniform1ui: None,
  glProgramUniform1uiv: None,
  glProgramUniform2d: None,
  glProgramUniform2dv: None,
  glProgramUniform2f: None,
  glProgramUniform2fv: None,
  glProgramUniform2i: None,
  glProgramUniform2iv: None,
  glProgramUniform2ui: None,
  glProgramUniform2uiv: None,
  glProgramUniform3d: None,
  glProgramUniform3dv: None,
  glProgramUniform3f: None,
  glProgramUniform3fv: None,
  glProgramUniform3i: None,
  glProgramUniform3iv: None,
  glProgramUniform3ui: None,
  glProgramUniform3uiv: None,
  glProgramUniform4d: None,
  glProgramUniform4dv: None,
  glProgramUniform4f: None,
  glProgramUniform4fv: None,
  glProgramUniform4i: None,
  glProgramUniform4iv: None,
  glProgramUniform4ui: None,
  glProgramUniform4uiv: None,
  glProgramUniformMatrix2dv: None,
  glProgramUniformMatrix2fv: None,
  glProgramUniformMatrix2x3dv: None,
  glProgramUniformMatrix2x3fv: None,
  glProgramUniformMatrix2x4dv: None,
  glProgramUniformMatrix2x4fv: None,
  glProgramUniformMatrix3dv: None,
  glProgramUniformMatrix3fv: None,
  glProgramUniformMatrix3x2dv: None,
  glProgramUniformMatrix3x2fv: None,
  glProgramUniformMatrix3x4dv: None,
  glProgramUniformMatrix3x4fv: None,
  glProgramUniformMatrix4dv: None,
  glProgramUniformMatrix4fv: None,
  glProgramUniformMatrix4x2dv: None,
  glProgramUniformMatrix4x2fv: None,
  glProgramUniformMatrix4x3dv: None,
  glProgramUniformMatrix4x3fv: None,
  glProvokingVertex: None,
  glPushDebugGroup: None,
  glPushDebugGroupKHR: None,
  glQueryCounter: None,
  glReadBuffer: None,
  glReadPixels: None,
  glReadnPixels: None,
  glReleaseShaderCompiler: None,
  glRenderbufferStorage: None,
  glRenderbufferStorageMultisample: None,
  glResumeTransformFeedback: None,
  glSampleCoverage: None,
  glSampleMaski: None,
  glSamplerParameterIiv: None,
  glSamplerParameterIuiv: None,
  glSamplerParameterf: None,
  glSamplerParameterfv: None,
  glSamplerParameteri: None,
  glSamplerParameteriv: None,
  glScissor: None,
  glScissorArrayv: None,
  glScissorIndexed: None,
  glScissorIndexedv: None,
  glSecondaryColorP3ui: None,
  glSecondaryColorP3uiv: None,
  glShaderBinary: None,
  glShaderSource: None,
  glShaderStorageBlockBinding: None,
  glSpecializeShader: None,
  glStencilFunc: None,
  glStencilFuncSeparate: None,
  glStencilMask: None,
  glStencilMaskSeparate: None,
  glStencilOp: None,
  glStencilOpSeparate: None,
  glTexBuffer: None,
  glTexBufferRange: None,
  glTexCoordP1ui: None,
  glTexCoordP1uiv: None,
  glTexCoordP2ui: None,
  glTexCoordP2uiv: None,
  glTexCoordP3ui: None,
  glTexCoordP3uiv: None,
  glTexCoordP4ui: None,
  glTexCoordP4uiv: None,
  glTexImage1D: None,
  glTexImage2D: None,
  glTexImage2DMultisample: None,
  glTexImage3D: None,
  glTexImage3DMultisample: None,
  glTexParameterIiv: None,
  glTexParameterIuiv: None,
  glTexParameterf: None,
  glTexParameterfv: None,
  glTexParameteri: None,
  glTexParameteriv: None,
  glTexStorage1D: None,
  glTexStorage2D: None,
  glTexStorage2DMultisample: None,
  glTexStorage3D: None,
  glTexStorage3DMultisample: None,
  glTexSubImage1D: None,
  glTexSubImage2D: None,
  glTexSubImage3D: None,
  glTextureBarrier: None,
  glTextureBuffer: None,
  glTextureBufferRange: None,
  glTextureParameterIiv: None,
  glTextureParameterIuiv: None,
  glTextureParameterf: None,
  glTextureParameterfv: None,
  glTextureParameteri: None,
  glTextureParameteriv: None,
  glTextureStorage1D: None,
  glTextureStorage2D: None,
  glTextureStorage2DMultisample: None,
  glTextureStorage3D: None,
  glTextureStorage3DMultisample: None,
  glTextureSubImage1D: None,
  glTextureSubImage2D: None,
  glTextureSubImage3D: None,
  glTextureView: None,
  glTransformFeedbackBufferBase: None,
  glTransformFeedbackBufferRange: None,
  glTransformFeedbackVaryings: None,
  glUniform1d: None,
  glUniform1dv: None,
  glUniform1f: None,
  glUniform1fv: None,
  glUniform1i: None,
  glUniform1iv: None,
  glUniform1ui: None,
  glUniform1uiv: None,
  glUniform2d: None,
  glUniform2dv: None,
  glUniform2f: None,
  glUniform2fv: None,
  glUniform2i: None,
  glUniform2iv: None,
  glUniform2ui: None,
  glUniform2uiv: None,
  glUniform3d: None,
  glUniform3dv: None,
  glUniform3f: None,
  glUniform3fv: None,
  glUniform3i: None,
  glUniform3iv: None,
  glUniform3ui: None,
  glUniform3uiv: None,
  glUniform4d: None,
  glUniform4dv: None,
  glUniform4f: None,
  glUniform4fv: None,
  glUniform4i: None,
  glUniform4iv: None,
  glUniform4ui: None,
  glUniform4uiv: None,
  glUniformBlockBinding: None,
  glUniformMatrix2dv: None,
  glUniformMatrix2fv: None,
  glUniformMatrix2x3dv: None,
  glUniformMatrix2x3fv: None,
  glUniformMatrix2x4dv: None,
  glUniformMatrix2x4fv: None,
  glUniformMatrix3dv: None,
  glUniformMatrix3fv: None,
  glUniformMatrix3x2dv: None,
  glUniformMatrix3x2fv: None,
  glUniformMatrix3x4dv: None,
  glUniformMatrix3x4fv: None,
  glUniformMatrix4dv: None,
  glUniformMatrix4fv: None,
  glUniformMatrix4x2dv: None,
  glUniformMatrix4x2fv: None,
  glUniformMatrix4x3dv: None,
  glUniformMatrix4x3fv: None,
  glUniformSubroutinesuiv: None,
  glUnmapBuffer: None,
  glUnmapNamedBuffer: None,
  glUseProgram: None,
  glUseProgramStages: None,
  glValidateProgram: None,
  glValidateProgramPipeline: None,
  glVertexArrayAttribBinding: None,
  glVertexArrayAttribFormat: None,
  glVertexArrayAttribIFormat: None,
  glVertexArrayAttribLFormat: None,
  glVertexArrayBindingDivisor: None,
  glVertexArrayElementBuffer: None,
  glVertexArrayVertexBuffer: None,
  glVertexArrayVertexBuffers: None,
  glVertexAttrib1d: None,
  glVertexAttrib1dv: None,
  glVertexAttrib1f: None,
  glVertexAttrib1fv: None,
  glVertexAttrib1s: None,
  glVertexAttrib1sv: None,
  glVertexAttrib2d: None,
  glVertexAttrib2dv: None,
  glVertexAttrib2f: None,
  glVertexAttrib2fv: None,
  glVertexAttrib2s: None,
  glVertexAttrib2sv: None,
  glVertexAttrib3d: None,
  glVertexAttrib3dv: None,
  glVertexAttrib3f: None,
  glVertexAttrib3fv: None,
  glVertexAttrib3s: None,
  glVertexAttrib3sv: None,
  glVertexAttrib4Nbv: None,
  glVertexAttrib4Niv: None,
  glVertexAttrib4Nsv: None,
  glVertexAttrib4Nub: None,
  glVertexAttrib4Nubv: None,
  glVertexAttrib4Nuiv: None,
  glVertexAttrib4Nusv: None,
  glVertexAttrib4bv: None,
  glVertexAttrib4d: None,
  glVertexAttrib4dv: None,
  glVertexAttrib4f: None,
  glVertexAttrib4fv: None,
  glVertexAttrib4iv: None,
  glVertexAttrib4s: None,
  glVertexAttrib4sv: None,
  glVertexAttrib4ubv: None,
  glVertexAttrib4uiv: None,
  glVertexAttrib4usv: None,
  glVertexAttribBinding: None,
  glVertexAttribDivisor: None,
  glVertexAttribFormat: None,
  glVertexAttribI1i: None,
  glVertexAttribI1iv: None,
  glVertexAttribI1ui: None,
  glVertexAttribI1uiv: None,
  glVertexAttribI2i: None,
  glVertexAttribI2iv: None,
  glVertexAttribI2ui: None,
  glVertexAttribI2uiv: None,
  glVertexAttribI3i: None,
  glVertexAttribI3iv: None,
  glVertexAttribI3ui: None,
  glVertexAttribI3uiv: None,
  glVertexAttribI4bv: None,
  glVertexAttribI4i: None,
  glVertexAttribI4iv: None,
  glVertexAttribI4sv: None,
  glVertexAttribI4ubv: None,
  glVertexAttribI4ui: None,
  glVertexAttribI4uiv: None,
  glVertexAttribI4usv: None,
  glVertexAttribIFormat: None,
  glVertexAttribIPointer: None,
  glVertexAttribL1d: None,
  glVertexAttribL1dv: None,
  glVertexAttribL2d: None,
  glVertexAttribL2dv: None,
  glVertexAttribL3d: None,
  glVertexAttribL3dv: None,
  glVertexAttribL4d: None,
  glVertexAttribL4dv: None,
  glVertexAttribLFormat: None,
  glVertexAttribLPointer: None,
  glVertexAttribP1ui: None,
  glVertexAttribP1uiv: None,
  glVertexAttribP2ui: None,
  glVertexAttribP2uiv: None,
  glVertexAttribP3ui: None,
  glVertexAttribP3uiv: None,
  glVertexAttribP4ui: None,
  glVertexAttribP4uiv: None,
  glVertexAttribPointer: None,
  glVertexBindingDivisor: None,
  glVertexP2ui: None,
  glVertexP2uiv: None,
  glVertexP3ui: None,
  glVertexP3uiv: None,
  glVertexP4ui: None,
  glVertexP4uiv: None,
  glViewport: None,
  glViewportArrayv: None,
  glViewportIndexedf: None,
  glViewportIndexedfv: None,
  glWaitSync: None,
};

#[cold]
#[doc(hidden)]
#[inline(never)]
#[cfg_attr(feature = "track_caller", track_caller)]
fn cold_panic(msg: &str) -> ! {
  panic!("Called a GL fn that wasn't loaded: {msg}");
}

macro_rules! mk_wrapper_method {
  ($full_name:ident, $short_name:ident, [$($arg_name:ident : $arg_ty:ty,)*] -> $ret_ty:ty) => {
    #[inline]
    #[allow(nonstandard_style)]
    #[allow(clippy::unused_unit)]
    #[allow(clippy::needless_return)]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::missing_safety_doc)]
    #[cfg_attr(feature = "track_caller", track_caller)]
    pub unsafe fn $short_name(&self, $($arg_name : $arg_ty),*) -> $ret_ty {
      if let Some(f) = self.$full_name {
        return unsafe { f($($arg_name),*) };
      } else {
        cold_panic(stringify!($full_name));
      }
    }
  };
}

impl GlFns {
  mk_wrapper_method!(glActiveShaderProgram, ActiveShaderProgram, [pipeline: GLuint, program: GLuint, ] -> ());
  mk_wrapper_method!(glActiveTexture, ActiveTexture, [texture: GLenum, ] -> ());
  mk_wrapper_method!(glAttachShader, AttachShader, [program: GLuint, shader: GLuint, ] -> ());
  mk_wrapper_method!(glBeginConditionalRender, BeginConditionalRender, [id: GLuint, mode: GLenum, ] -> ());
  mk_wrapper_method!(glBeginQuery, BeginQuery, [target: GLenum, id: GLuint, ] -> ());
  mk_wrapper_method!(glBeginQueryIndexed, BeginQueryIndexed, [target: GLenum, index: GLuint, id: GLuint, ] -> ());
  mk_wrapper_method!(glBeginTransformFeedback, BeginTransformFeedback, [primitiveMode: GLenum, ] -> ());
  mk_wrapper_method!(glBindAttribLocation, BindAttribLocation, [program: GLuint, index: GLuint, name: *const GLchar, ] -> ());
  mk_wrapper_method!(glBindBuffer, BindBuffer, [target: GLenum, buffer: GLuint, ] -> ());
  mk_wrapper_method!(glBindBufferBase, BindBufferBase, [target: GLenum, index: GLuint, buffer: GLuint, ] -> ());
  mk_wrapper_method!(glBindBufferRange, BindBufferRange, [target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glBindBuffersBase, BindBuffersBase, [target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, ] -> ());
  mk_wrapper_method!(glBindBuffersRange, BindBuffersRange, [target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr, ] -> ());
  mk_wrapper_method!(glBindFragDataLocation, BindFragDataLocation, [program: GLuint, color: GLuint, name: *const GLchar, ] -> ());
  mk_wrapper_method!(glBindFragDataLocationIndexed, BindFragDataLocationIndexed, [program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar, ] -> ());
  mk_wrapper_method!(glBindFramebuffer, BindFramebuffer, [target: GLenum, framebuffer: GLuint, ] -> ());
  mk_wrapper_method!(glBindImageTexture, BindImageTexture, [unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum, ] -> ());
  mk_wrapper_method!(glBindImageTextures, BindImageTextures, [first: GLuint, count: GLsizei, textures: *const GLuint, ] -> ());
  mk_wrapper_method!(glBindProgramPipeline, BindProgramPipeline, [pipeline: GLuint, ] -> ());
  mk_wrapper_method!(glBindRenderbuffer, BindRenderbuffer, [target: GLenum, renderbuffer: GLuint, ] -> ());
  mk_wrapper_method!(glBindSampler, BindSampler, [unit: GLuint, sampler: GLuint, ] -> ());
  mk_wrapper_method!(glBindSamplers, BindSamplers, [first: GLuint, count: GLsizei, samplers: *const GLuint, ] -> ());
  mk_wrapper_method!(glBindTexture, BindTexture, [target: GLenum, texture: GLuint, ] -> ());
  mk_wrapper_method!(glBindTextureUnit, BindTextureUnit, [unit: GLuint, texture: GLuint, ] -> ());
  mk_wrapper_method!(glBindTextures, BindTextures, [first: GLuint, count: GLsizei, textures: *const GLuint, ] -> ());
  mk_wrapper_method!(glBindTransformFeedback, BindTransformFeedback, [target: GLenum, id: GLuint, ] -> ());
  mk_wrapper_method!(glBindVertexArray, BindVertexArray, [array: GLuint, ] -> ());
  mk_wrapper_method!(glBindVertexBuffer, BindVertexBuffer, [bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei, ] -> ());
  mk_wrapper_method!(glBindVertexBuffers, BindVertexBuffers, [first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei, ] -> ());
  mk_wrapper_method!(glBlendBarrier, BlendBarrier, [] -> ());
  mk_wrapper_method!(glBlendColor, BlendColor, [red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat, ] -> ());
  mk_wrapper_method!(glBlendEquation, BlendEquation, [mode: GLenum, ] -> ());
  mk_wrapper_method!(glBlendEquationSeparate, BlendEquationSeparate, [modeRGB: GLenum, modeAlpha: GLenum, ] -> ());
  mk_wrapper_method!(glBlendEquationSeparatei, BlendEquationSeparatei, [buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum, ] -> ());
  mk_wrapper_method!(glBlendEquationi, BlendEquationi, [buf: GLuint, mode: GLenum, ] -> ());
  mk_wrapper_method!(glBlendFunc, BlendFunc, [sfactor: GLenum, dfactor: GLenum, ] -> ());
  mk_wrapper_method!(glBlendFuncSeparate, BlendFuncSeparate, [sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum, ] -> ());
  mk_wrapper_method!(glBlendFuncSeparatei, BlendFuncSeparatei, [buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum, ] -> ());
  mk_wrapper_method!(glBlendFunci, BlendFunci, [buf: GLuint, src: GLenum, dst: GLenum, ] -> ());
  mk_wrapper_method!(glBlitFramebuffer, BlitFramebuffer, [srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum, ] -> ());
  mk_wrapper_method!(glBlitNamedFramebuffer, BlitNamedFramebuffer, [readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum, ] -> ());
  mk_wrapper_method!(glBufferData, BufferData, [target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum, ] -> ());
  mk_wrapper_method!(glBufferStorage, BufferStorage, [target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield, ] -> ());
  mk_wrapper_method!(glBufferSubData, BufferSubData, [target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCheckFramebufferStatus, CheckFramebufferStatus, [target: GLenum, ] -> GLenum);
  mk_wrapper_method!(glCheckNamedFramebufferStatus, CheckNamedFramebufferStatus, [framebuffer: GLuint, target: GLenum, ] -> GLenum);
  mk_wrapper_method!(glClampColor, ClampColor, [target: GLenum, clamp: GLenum, ] -> ());
  mk_wrapper_method!(glClear, Clear, [mask: GLbitfield, ] -> ());
  mk_wrapper_method!(glClearBufferData, ClearBufferData, [target: GLenum, internalformat: GLenum, format: GLenum, ty: GLenum, data: *const c_void, ] -> ());
  mk_wrapper_method!(glClearBufferSubData, ClearBufferSubData, [target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, ty: GLenum, data: *const c_void, ] -> ());
  mk_wrapper_method!(glClearBufferfi, ClearBufferfi, [buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint, ] -> ());
  mk_wrapper_method!(glClearBufferfv, ClearBufferfv, [buffer: GLenum, drawbuffer: GLint, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glClearBufferiv, ClearBufferiv, [buffer: GLenum, drawbuffer: GLint, value: *const GLint, ] -> ());
  mk_wrapper_method!(glClearBufferuiv, ClearBufferuiv, [buffer: GLenum, drawbuffer: GLint, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glClearColor, ClearColor, [red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat, ] -> ());
  mk_wrapper_method!(glClearDepth, ClearDepth, [depth: GLdouble, ] -> ());
  mk_wrapper_method!(glClearDepthf, ClearDepthf, [d: GLfloat, ] -> ());
  mk_wrapper_method!(glClearNamedBufferData, ClearNamedBufferData, [buffer: GLuint, internalformat: GLenum, format: GLenum, ty: GLenum, data: *const c_void, ] -> ());
  mk_wrapper_method!(glClearNamedBufferSubData, ClearNamedBufferSubData, [buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, ty: GLenum, data: *const c_void, ] -> ());
  mk_wrapper_method!(glClearNamedFramebufferfi, ClearNamedFramebufferfi, [framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint, ] -> ());
  mk_wrapper_method!(glClearNamedFramebufferfv, ClearNamedFramebufferfv, [framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glClearNamedFramebufferiv, ClearNamedFramebufferiv, [framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint, ] -> ());
  mk_wrapper_method!(glClearNamedFramebufferuiv, ClearNamedFramebufferuiv, [framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glClearStencil, ClearStencil, [s: GLint, ] -> ());
  mk_wrapper_method!(glClearTexImage, ClearTexImage, [texture: GLuint, level: GLint, format: GLenum, ty: GLenum, data: *const c_void, ] -> ());
  mk_wrapper_method!(glClearTexSubImage, ClearTexSubImage, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, ty: GLenum, data: *const c_void, ] -> ());
  mk_wrapper_method!(glClientWaitSync, ClientWaitSync, [sync: GLsync, flags: GLbitfield, timeout: GLuint64, ] -> GLenum);
  mk_wrapper_method!(glClipControl, ClipControl, [origin: GLenum, depth: GLenum, ] -> ());
  mk_wrapper_method!(glColorMask, ColorMask, [red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean, ] -> ());
  mk_wrapper_method!(glColorMaski, ColorMaski, [index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean, ] -> ());
  mk_wrapper_method!(glColorP3ui, ColorP3ui, [ty: GLenum, color: GLuint, ] -> ());
  mk_wrapper_method!(glColorP3uiv, ColorP3uiv, [ty: GLenum, color: *const GLuint, ] -> ());
  mk_wrapper_method!(glColorP4ui, ColorP4ui, [ty: GLenum, color: GLuint, ] -> ());
  mk_wrapper_method!(glColorP4uiv, ColorP4uiv, [ty: GLenum, color: *const GLuint, ] -> ());
  mk_wrapper_method!(glCompileShader, CompileShader, [shader: GLuint, ] -> ());
  mk_wrapper_method!(glCompressedTexImage1D, CompressedTexImage1D, [target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTexImage2D, CompressedTexImage2D, [target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTexImage3D, CompressedTexImage3D, [target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTexSubImage1D, CompressedTexSubImage1D, [target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTexSubImage2D, CompressedTexSubImage2D, [target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTexSubImage3D, CompressedTexSubImage3D, [target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTextureSubImage1D, CompressedTextureSubImage1D, [texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTextureSubImage2D, CompressedTextureSubImage2D, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCompressedTextureSubImage3D, CompressedTextureSubImage3D, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void, ] -> ());
  mk_wrapper_method!(glCopyBufferSubData, CopyBufferSubData, [readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glCopyImageSubData, CopyImageSubData, [srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei, ] -> ());
  mk_wrapper_method!(glCopyNamedBufferSubData, CopyNamedBufferSubData, [readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glCopyTexImage1D, CopyTexImage1D, [target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint, ] -> ());
  mk_wrapper_method!(glCopyTexImage2D, CopyTexImage2D, [target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint, ] -> ());
  mk_wrapper_method!(glCopyTexSubImage1D, CopyTexSubImage1D, [target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei, ] -> ());
  mk_wrapper_method!(glCopyTexSubImage2D, CopyTexSubImage2D, [target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glCopyTexSubImage3D, CopyTexSubImage3D, [target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glCopyTextureSubImage1D, CopyTextureSubImage1D, [texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei, ] -> ());
  mk_wrapper_method!(glCopyTextureSubImage2D, CopyTextureSubImage2D, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glCopyTextureSubImage3D, CopyTextureSubImage3D, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glCreateBuffers, CreateBuffers, [n: GLsizei, buffers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateFramebuffers, CreateFramebuffers, [n: GLsizei, framebuffers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateProgram, CreateProgram, [] -> GLuint);
  mk_wrapper_method!(glCreateProgramPipelines, CreateProgramPipelines, [n: GLsizei, pipelines: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateQueries, CreateQueries, [target: GLenum, n: GLsizei, ids: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateRenderbuffers, CreateRenderbuffers, [n: GLsizei, renderbuffers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateSamplers, CreateSamplers, [n: GLsizei, samplers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateShader, CreateShader, [ty: GLenum, ] -> GLuint);
  mk_wrapper_method!(glCreateShaderProgramv, CreateShaderProgramv, [ty: GLenum, count: GLsizei, strings: *const *const GLchar, ] -> GLuint);
  mk_wrapper_method!(glCreateTextures, CreateTextures, [target: GLenum, n: GLsizei, textures: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateTransformFeedbacks, CreateTransformFeedbacks, [n: GLsizei, ids: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCreateVertexArrays, CreateVertexArrays, [n: GLsizei, arrays: *mut GLuint, ] -> ());
  mk_wrapper_method!(glCullFace, CullFace, [mode: GLenum, ] -> ());
  mk_wrapper_method!(glDebugMessageCallback, DebugMessageCallback, [callback: GLDEBUGPROC, userParam: *const c_void, ] -> ());
  mk_wrapper_method!(glDebugMessageCallbackKHR, DebugMessageCallbackKHR, [callback: GLDEBUGPROCKHR, userParam: *const c_void, ] -> ());
  mk_wrapper_method!(glDebugMessageControl, DebugMessageControl, [source: GLenum, ty: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean, ] -> ());
  mk_wrapper_method!(glDebugMessageControlKHR, DebugMessageControlKHR, [source: GLenum, ty: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean, ] -> ());
  mk_wrapper_method!(glDebugMessageInsert, DebugMessageInsert, [source: GLenum, ty: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar, ] -> ());
  mk_wrapper_method!(glDebugMessageInsertKHR, DebugMessageInsertKHR, [source: GLenum, ty: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar, ] -> ());
  mk_wrapper_method!(glDeleteBuffers, DeleteBuffers, [n: GLsizei, buffers: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteFramebuffers, DeleteFramebuffers, [n: GLsizei, framebuffers: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteProgram, DeleteProgram, [program: GLuint, ] -> ());
  mk_wrapper_method!(glDeleteProgramPipelines, DeleteProgramPipelines, [n: GLsizei, pipelines: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteQueries, DeleteQueries, [n: GLsizei, ids: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteRenderbuffers, DeleteRenderbuffers, [n: GLsizei, renderbuffers: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteSamplers, DeleteSamplers, [count: GLsizei, samplers: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteShader, DeleteShader, [shader: GLuint, ] -> ());
  mk_wrapper_method!(glDeleteSync, DeleteSync, [sync: GLsync, ] -> ());
  mk_wrapper_method!(glDeleteTextures, DeleteTextures, [n: GLsizei, textures: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteTransformFeedbacks, DeleteTransformFeedbacks, [n: GLsizei, ids: *const GLuint, ] -> ());
  mk_wrapper_method!(glDeleteVertexArrays, DeleteVertexArrays, [n: GLsizei, arrays: *const GLuint, ] -> ());
  mk_wrapper_method!(glDepthFunc, DepthFunc, [func: GLenum, ] -> ());
  mk_wrapper_method!(glDepthMask, DepthMask, [flag: GLboolean, ] -> ());
  mk_wrapper_method!(glDepthRange, DepthRange, [n: GLdouble, f: GLdouble, ] -> ());
  mk_wrapper_method!(glDepthRangeArrayv, DepthRangeArrayv, [first: GLuint, count: GLsizei, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glDepthRangeIndexed, DepthRangeIndexed, [index: GLuint, n: GLdouble, f: GLdouble, ] -> ());
  mk_wrapper_method!(glDepthRangef, DepthRangef, [n: GLfloat, f: GLfloat, ] -> ());
  mk_wrapper_method!(glDetachShader, DetachShader, [program: GLuint, shader: GLuint, ] -> ());
  mk_wrapper_method!(glDisable, Disable, [cap: GLenum, ] -> ());
  mk_wrapper_method!(glDisableVertexArrayAttrib, DisableVertexArrayAttrib, [vaobj: GLuint, index: GLuint, ] -> ());
  mk_wrapper_method!(glDisableVertexAttribArray, DisableVertexAttribArray, [index: GLuint, ] -> ());
  mk_wrapper_method!(glDisablei, Disablei, [target: GLenum, index: GLuint, ] -> ());
  mk_wrapper_method!(glDispatchCompute, DispatchCompute, [num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint, ] -> ());
  mk_wrapper_method!(glDispatchComputeIndirect, DispatchComputeIndirect, [indirect: GLintptr, ] -> ());
  mk_wrapper_method!(glDrawArrays, DrawArrays, [mode: GLenum, first: GLint, count: GLsizei, ] -> ());
  mk_wrapper_method!(glDrawArraysIndirect, DrawArraysIndirect, [mode: GLenum, indirect: *const c_void, ] -> ());
  mk_wrapper_method!(glDrawArraysInstanced, DrawArraysInstanced, [mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, ] -> ());
  mk_wrapper_method!(glDrawArraysInstancedBaseInstance, DrawArraysInstancedBaseInstance, [mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint, ] -> ());
  mk_wrapper_method!(glDrawBuffer, DrawBuffer, [buf: GLenum, ] -> ());
  mk_wrapper_method!(glDrawBuffers, DrawBuffers, [n: GLsizei, bufs: *const GLenum, ] -> ());
  mk_wrapper_method!(glDrawElements, DrawElements, [mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void, ] -> ());
  mk_wrapper_method!(glDrawElementsBaseVertex, DrawElementsBaseVertex, [mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void, basevertex: GLint, ] -> ());
  mk_wrapper_method!(glDrawElementsIndirect, DrawElementsIndirect, [mode: GLenum, ty: GLenum, indirect: *const c_void, ] -> ());
  mk_wrapper_method!(glDrawElementsInstanced, DrawElementsInstanced, [mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void, instancecount: GLsizei, ] -> ());
  mk_wrapper_method!(glDrawElementsInstancedBaseInstance, DrawElementsInstancedBaseInstance, [mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint, ] -> ());
  mk_wrapper_method!(glDrawElementsInstancedBaseVertex, DrawElementsInstancedBaseVertex, [mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, ] -> ());
  mk_wrapper_method!(glDrawElementsInstancedBaseVertexBaseInstance, DrawElementsInstancedBaseVertexBaseInstance, [mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint, ] -> ());
  mk_wrapper_method!(glDrawRangeElements, DrawRangeElements, [mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, ty: GLenum, indices: *const c_void, ] -> ());
  mk_wrapper_method!(glDrawRangeElementsBaseVertex, DrawRangeElementsBaseVertex, [mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, ty: GLenum, indices: *const c_void, basevertex: GLint, ] -> ());
  mk_wrapper_method!(glDrawTransformFeedback, DrawTransformFeedback, [mode: GLenum, id: GLuint, ] -> ());
  mk_wrapper_method!(glDrawTransformFeedbackInstanced, DrawTransformFeedbackInstanced, [mode: GLenum, id: GLuint, instancecount: GLsizei, ] -> ());
  mk_wrapper_method!(glDrawTransformFeedbackStream, DrawTransformFeedbackStream, [mode: GLenum, id: GLuint, stream: GLuint, ] -> ());
  mk_wrapper_method!(glDrawTransformFeedbackStreamInstanced, DrawTransformFeedbackStreamInstanced, [mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei, ] -> ());
  mk_wrapper_method!(glEnable, Enable, [cap: GLenum, ] -> ());
  mk_wrapper_method!(glEnableVertexArrayAttrib, EnableVertexArrayAttrib, [vaobj: GLuint, index: GLuint, ] -> ());
  mk_wrapper_method!(glEnableVertexAttribArray, EnableVertexAttribArray, [index: GLuint, ] -> ());
  mk_wrapper_method!(glEnablei, Enablei, [target: GLenum, index: GLuint, ] -> ());
  mk_wrapper_method!(glEndConditionalRender, EndConditionalRender, [] -> ());
  mk_wrapper_method!(glEndQuery, EndQuery, [target: GLenum, ] -> ());
  mk_wrapper_method!(glEndQueryIndexed, EndQueryIndexed, [target: GLenum, index: GLuint, ] -> ());
  mk_wrapper_method!(glEndTransformFeedback, EndTransformFeedback, [] -> ());
  mk_wrapper_method!(glFenceSync, FenceSync, [condition: GLenum, flags: GLbitfield, ] -> GLsync);
  mk_wrapper_method!(glFinish, Finish, [] -> ());
  mk_wrapper_method!(glFlush, Flush, [] -> ());
  mk_wrapper_method!(glFlushMappedBufferRange, FlushMappedBufferRange, [target: GLenum, offset: GLintptr, length: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glFlushMappedNamedBufferRange, FlushMappedNamedBufferRange, [buffer: GLuint, offset: GLintptr, length: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glFramebufferParameteri, FramebufferParameteri, [target: GLenum, pname: GLenum, param: GLint, ] -> ());
  mk_wrapper_method!(glFramebufferRenderbuffer, FramebufferRenderbuffer, [target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint, ] -> ());
  mk_wrapper_method!(glFramebufferTexture, FramebufferTexture, [target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, ] -> ());
  mk_wrapper_method!(glFramebufferTexture1D, FramebufferTexture1D, [target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, ] -> ());
  mk_wrapper_method!(glFramebufferTexture2D, FramebufferTexture2D, [target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, ] -> ());
  mk_wrapper_method!(glFramebufferTexture3D, FramebufferTexture3D, [target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint, ] -> ());
  mk_wrapper_method!(glFramebufferTextureLayer, FramebufferTextureLayer, [target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint, ] -> ());
  mk_wrapper_method!(glFrontFace, FrontFace, [mode: GLenum, ] -> ());
  mk_wrapper_method!(glGenBuffers, GenBuffers, [n: GLsizei, buffers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenFramebuffers, GenFramebuffers, [n: GLsizei, framebuffers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenProgramPipelines, GenProgramPipelines, [n: GLsizei, pipelines: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenQueries, GenQueries, [n: GLsizei, ids: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenRenderbuffers, GenRenderbuffers, [n: GLsizei, renderbuffers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenSamplers, GenSamplers, [count: GLsizei, samplers: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenTextures, GenTextures, [n: GLsizei, textures: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenTransformFeedbacks, GenTransformFeedbacks, [n: GLsizei, ids: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenVertexArrays, GenVertexArrays, [n: GLsizei, arrays: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGenerateMipmap, GenerateMipmap, [target: GLenum, ] -> ());
  mk_wrapper_method!(glGenerateTextureMipmap, GenerateTextureMipmap, [texture: GLuint, ] -> ());
  mk_wrapper_method!(glGetActiveAtomicCounterBufferiv, GetActiveAtomicCounterBufferiv, [program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetActiveAttrib, GetActiveAttrib, [program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, ty: *mut GLenum, name: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetActiveSubroutineName, GetActiveSubroutineName, [program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetActiveSubroutineUniformName, GetActiveSubroutineUniformName, [program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetActiveSubroutineUniformiv, GetActiveSubroutineUniformiv, [program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetActiveUniform, GetActiveUniform, [program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, ty: *mut GLenum, name: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetActiveUniformBlockName, GetActiveUniformBlockName, [program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetActiveUniformBlockiv, GetActiveUniformBlockiv, [program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetActiveUniformName, GetActiveUniformName, [program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetActiveUniformsiv, GetActiveUniformsiv, [program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetAttachedShaders, GetAttachedShaders, [program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetAttribLocation, GetAttribLocation, [program: GLuint, name: *const GLchar, ] -> GLint);
  mk_wrapper_method!(glGetBooleani_v, GetBooleani_v, [target: GLenum, index: GLuint, data: *mut GLboolean, ] -> ());
  mk_wrapper_method!(glGetBooleanv, GetBooleanv, [pname: GLenum, data: *mut GLboolean, ] -> ());
  mk_wrapper_method!(glGetBufferParameteri64v, GetBufferParameteri64v, [target: GLenum, pname: GLenum, params: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetBufferParameteriv, GetBufferParameteriv, [target: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetBufferPointerv, GetBufferPointerv, [target: GLenum, pname: GLenum, params: *mut *mut c_void, ] -> ());
  mk_wrapper_method!(glGetBufferSubData, GetBufferSubData, [target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetCompressedTexImage, GetCompressedTexImage, [target: GLenum, level: GLint, img: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetCompressedTextureImage, GetCompressedTextureImage, [texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetCompressedTextureSubImage, GetCompressedTextureSubImage, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetDebugMessageLog, GetDebugMessageLog, [count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar, ] -> GLuint);
  mk_wrapper_method!(glGetDebugMessageLogKHR, GetDebugMessageLogKHR, [count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar, ] -> GLuint);
  mk_wrapper_method!(glGetDoublei_v, GetDoublei_v, [target: GLenum, index: GLuint, data: *mut GLdouble, ] -> ());
  mk_wrapper_method!(glGetDoublev, GetDoublev, [pname: GLenum, data: *mut GLdouble, ] -> ());
  mk_wrapper_method!(glGetError, GetError, [] -> GLenum);
  mk_wrapper_method!(glGetFloati_v, GetFloati_v, [target: GLenum, index: GLuint, data: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetFloatv, GetFloatv, [pname: GLenum, data: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetFragDataIndex, GetFragDataIndex, [program: GLuint, name: *const GLchar, ] -> GLint);
  mk_wrapper_method!(glGetFragDataLocation, GetFragDataLocation, [program: GLuint, name: *const GLchar, ] -> GLint);
  mk_wrapper_method!(glGetFramebufferAttachmentParameteriv, GetFramebufferAttachmentParameteriv, [target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetFramebufferParameteriv, GetFramebufferParameteriv, [target: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetGraphicsResetStatus, GetGraphicsResetStatus, [] -> GLenum);
  mk_wrapper_method!(glGetInteger64i_v, GetInteger64i_v, [target: GLenum, index: GLuint, data: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetInteger64v, GetInteger64v, [pname: GLenum, data: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetIntegeri_v, GetIntegeri_v, [target: GLenum, index: GLuint, data: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetIntegerv, GetIntegerv, [pname: GLenum, data: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetInternalformati64v, GetInternalformati64v, [target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetInternalformativ, GetInternalformativ, [target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetMultisamplefv, GetMultisamplefv, [pname: GLenum, index: GLuint, val: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetNamedBufferParameteri64v, GetNamedBufferParameteri64v, [buffer: GLuint, pname: GLenum, params: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetNamedBufferParameteriv, GetNamedBufferParameteriv, [buffer: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetNamedBufferPointerv, GetNamedBufferPointerv, [buffer: GLuint, pname: GLenum, params: *mut *mut c_void, ] -> ());
  mk_wrapper_method!(glGetNamedBufferSubData, GetNamedBufferSubData, [buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetNamedFramebufferAttachmentParameteriv, GetNamedFramebufferAttachmentParameteriv, [framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetNamedFramebufferParameteriv, GetNamedFramebufferParameteriv, [framebuffer: GLuint, pname: GLenum, param: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetNamedRenderbufferParameteriv, GetNamedRenderbufferParameteriv, [renderbuffer: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetObjectLabel, GetObjectLabel, [identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetObjectLabelKHR, GetObjectLabelKHR, [identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetObjectPtrLabel, GetObjectPtrLabel, [ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetObjectPtrLabelKHR, GetObjectPtrLabelKHR, [ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetPointerv, GetPointerv, [pname: GLenum, params: *mut *mut c_void, ] -> ());
  mk_wrapper_method!(glGetPointervKHR, GetPointervKHR, [pname: GLenum, params: *mut *mut c_void, ] -> ());
  mk_wrapper_method!(glGetProgramBinary, GetProgramBinary, [program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetProgramInfoLog, GetProgramInfoLog, [program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetProgramInterfaceiv, GetProgramInterfaceiv, [program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetProgramPipelineInfoLog, GetProgramPipelineInfoLog, [pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetProgramPipelineiv, GetProgramPipelineiv, [pipeline: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetProgramResourceIndex, GetProgramResourceIndex, [program: GLuint, programInterface: GLenum, name: *const GLchar, ] -> GLuint);
  mk_wrapper_method!(glGetProgramResourceLocation, GetProgramResourceLocation, [program: GLuint, programInterface: GLenum, name: *const GLchar, ] -> GLint);
  mk_wrapper_method!(glGetProgramResourceLocationIndex, GetProgramResourceLocationIndex, [program: GLuint, programInterface: GLenum, name: *const GLchar, ] -> GLint);
  mk_wrapper_method!(glGetProgramResourceName, GetProgramResourceName, [program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetProgramResourceiv, GetProgramResourceiv, [program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetProgramStageiv, GetProgramStageiv, [program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetProgramiv, GetProgramiv, [program: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetQueryBufferObjecti64v, GetQueryBufferObjecti64v, [id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr, ] -> ());
  mk_wrapper_method!(glGetQueryBufferObjectiv, GetQueryBufferObjectiv, [id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr, ] -> ());
  mk_wrapper_method!(glGetQueryBufferObjectui64v, GetQueryBufferObjectui64v, [id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr, ] -> ());
  mk_wrapper_method!(glGetQueryBufferObjectuiv, GetQueryBufferObjectuiv, [id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr, ] -> ());
  mk_wrapper_method!(glGetQueryIndexediv, GetQueryIndexediv, [target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetQueryObjecti64v, GetQueryObjecti64v, [id: GLuint, pname: GLenum, params: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetQueryObjectiv, GetQueryObjectiv, [id: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetQueryObjectui64v, GetQueryObjectui64v, [id: GLuint, pname: GLenum, params: *mut GLuint64, ] -> ());
  mk_wrapper_method!(glGetQueryObjectuiv, GetQueryObjectuiv, [id: GLuint, pname: GLenum, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetQueryiv, GetQueryiv, [target: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetRenderbufferParameteriv, GetRenderbufferParameteriv, [target: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetSamplerParameterIiv, GetSamplerParameterIiv, [sampler: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetSamplerParameterIuiv, GetSamplerParameterIuiv, [sampler: GLuint, pname: GLenum, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetSamplerParameterfv, GetSamplerParameterfv, [sampler: GLuint, pname: GLenum, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetSamplerParameteriv, GetSamplerParameteriv, [sampler: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetShaderInfoLog, GetShaderInfoLog, [shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetShaderPrecisionFormat, GetShaderPrecisionFormat, [shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetShaderSource, GetShaderSource, [shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetShaderiv, GetShaderiv, [shader: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetString, GetString, [name: GLenum, ] -> *const GLubyte);
  mk_wrapper_method!(glGetStringi, GetStringi, [name: GLenum, index: GLuint, ] -> *const GLubyte);
  mk_wrapper_method!(glGetSubroutineIndex, GetSubroutineIndex, [program: GLuint, shadertype: GLenum, name: *const GLchar, ] -> GLuint);
  mk_wrapper_method!(glGetSubroutineUniformLocation, GetSubroutineUniformLocation, [program: GLuint, shadertype: GLenum, name: *const GLchar, ] -> GLint);
  mk_wrapper_method!(glGetSynciv, GetSynciv, [sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTexImage, GetTexImage, [target: GLenum, level: GLint, format: GLenum, ty: GLenum, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetTexLevelParameterfv, GetTexLevelParameterfv, [target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetTexLevelParameteriv, GetTexLevelParameteriv, [target: GLenum, level: GLint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTexParameterIiv, GetTexParameterIiv, [target: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTexParameterIuiv, GetTexParameterIuiv, [target: GLenum, pname: GLenum, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetTexParameterfv, GetTexParameterfv, [target: GLenum, pname: GLenum, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetTexParameteriv, GetTexParameteriv, [target: GLenum, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTextureImage, GetTextureImage, [texture: GLuint, level: GLint, format: GLenum, ty: GLenum, bufSize: GLsizei, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetTextureLevelParameterfv, GetTextureLevelParameterfv, [texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetTextureLevelParameteriv, GetTextureLevelParameteriv, [texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTextureParameterIiv, GetTextureParameterIiv, [texture: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTextureParameterIuiv, GetTextureParameterIuiv, [texture: GLuint, pname: GLenum, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetTextureParameterfv, GetTextureParameterfv, [texture: GLuint, pname: GLenum, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetTextureParameteriv, GetTextureParameteriv, [texture: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTextureSubImage, GetTextureSubImage, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, ty: GLenum, bufSize: GLsizei, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetTransformFeedbackVarying, GetTransformFeedbackVarying, [program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, ty: *mut GLenum, name: *mut GLchar, ] -> ());
  mk_wrapper_method!(glGetTransformFeedbacki64_v, GetTransformFeedbacki64_v, [xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetTransformFeedbacki_v, GetTransformFeedbacki_v, [xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetTransformFeedbackiv, GetTransformFeedbackiv, [xfb: GLuint, pname: GLenum, param: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetUniformBlockIndex, GetUniformBlockIndex, [program: GLuint, uniformBlockName: *const GLchar, ] -> GLuint);
  mk_wrapper_method!(glGetUniformIndices, GetUniformIndices, [program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetUniformLocation, GetUniformLocation, [program: GLuint, name: *const GLchar, ] -> GLint);
  mk_wrapper_method!(glGetUniformSubroutineuiv, GetUniformSubroutineuiv, [shadertype: GLenum, location: GLint, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetUniformdv, GetUniformdv, [program: GLuint, location: GLint, params: *mut GLdouble, ] -> ());
  mk_wrapper_method!(glGetUniformfv, GetUniformfv, [program: GLuint, location: GLint, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetUniformiv, GetUniformiv, [program: GLuint, location: GLint, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetUniformuiv, GetUniformuiv, [program: GLuint, location: GLint, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetVertexArrayIndexed64iv, GetVertexArrayIndexed64iv, [vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64, ] -> ());
  mk_wrapper_method!(glGetVertexArrayIndexediv, GetVertexArrayIndexediv, [vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetVertexArrayiv, GetVertexArrayiv, [vaobj: GLuint, pname: GLenum, param: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetVertexAttribIiv, GetVertexAttribIiv, [index: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetVertexAttribIuiv, GetVertexAttribIuiv, [index: GLuint, pname: GLenum, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetVertexAttribLdv, GetVertexAttribLdv, [index: GLuint, pname: GLenum, params: *mut GLdouble, ] -> ());
  mk_wrapper_method!(glGetVertexAttribPointerv, GetVertexAttribPointerv, [index: GLuint, pname: GLenum, pointer: *mut *mut c_void, ] -> ());
  mk_wrapper_method!(glGetVertexAttribdv, GetVertexAttribdv, [index: GLuint, pname: GLenum, params: *mut GLdouble, ] -> ());
  mk_wrapper_method!(glGetVertexAttribfv, GetVertexAttribfv, [index: GLuint, pname: GLenum, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetVertexAttribiv, GetVertexAttribiv, [index: GLuint, pname: GLenum, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetnColorTable, GetnColorTable, [target: GLenum, format: GLenum, ty: GLenum, bufSize: GLsizei, table: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetnCompressedTexImage, GetnCompressedTexImage, [target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetnConvolutionFilter, GetnConvolutionFilter, [target: GLenum, format: GLenum, ty: GLenum, bufSize: GLsizei, image: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetnHistogram, GetnHistogram, [target: GLenum, reset: GLboolean, format: GLenum, ty: GLenum, bufSize: GLsizei, values: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetnMapdv, GetnMapdv, [target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble, ] -> ());
  mk_wrapper_method!(glGetnMapfv, GetnMapfv, [target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetnMapiv, GetnMapiv, [target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetnMinmax, GetnMinmax, [target: GLenum, reset: GLboolean, format: GLenum, ty: GLenum, bufSize: GLsizei, values: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetnPixelMapfv, GetnPixelMapfv, [map: GLenum, bufSize: GLsizei, values: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetnPixelMapuiv, GetnPixelMapuiv, [map: GLenum, bufSize: GLsizei, values: *mut GLuint, ] -> ());
  mk_wrapper_method!(glGetnPixelMapusv, GetnPixelMapusv, [map: GLenum, bufSize: GLsizei, values: *mut GLushort, ] -> ());
  mk_wrapper_method!(glGetnPolygonStipple, GetnPolygonStipple, [bufSize: GLsizei, pattern: *mut GLubyte, ] -> ());
  mk_wrapper_method!(glGetnSeparableFilter, GetnSeparableFilter, [target: GLenum, format: GLenum, ty: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetnTexImage, GetnTexImage, [target: GLenum, level: GLint, format: GLenum, ty: GLenum, bufSize: GLsizei, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glGetnUniformdv, GetnUniformdv, [program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble, ] -> ());
  mk_wrapper_method!(glGetnUniformfv, GetnUniformfv, [program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat, ] -> ());
  mk_wrapper_method!(glGetnUniformiv, GetnUniformiv, [program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint, ] -> ());
  mk_wrapper_method!(glGetnUniformuiv, GetnUniformuiv, [program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint, ] -> ());
  mk_wrapper_method!(glHint, Hint, [target: GLenum, mode: GLenum, ] -> ());
  mk_wrapper_method!(glInvalidateBufferData, InvalidateBufferData, [buffer: GLuint, ] -> ());
  mk_wrapper_method!(glInvalidateBufferSubData, InvalidateBufferSubData, [buffer: GLuint, offset: GLintptr, length: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glInvalidateFramebuffer, InvalidateFramebuffer, [target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, ] -> ());
  mk_wrapper_method!(glInvalidateNamedFramebufferData, InvalidateNamedFramebufferData, [framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, ] -> ());
  mk_wrapper_method!(glInvalidateNamedFramebufferSubData, InvalidateNamedFramebufferSubData, [framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glInvalidateSubFramebuffer, InvalidateSubFramebuffer, [target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glInvalidateTexImage, InvalidateTexImage, [texture: GLuint, level: GLint, ] -> ());
  mk_wrapper_method!(glInvalidateTexSubImage, InvalidateTexSubImage, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, ] -> ());
  mk_wrapper_method!(glIsBuffer, IsBuffer, [buffer: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsEnabled, IsEnabled, [cap: GLenum, ] -> GLboolean);
  mk_wrapper_method!(glIsEnabledi, IsEnabledi, [target: GLenum, index: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsFramebuffer, IsFramebuffer, [framebuffer: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsProgram, IsProgram, [program: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsProgramPipeline, IsProgramPipeline, [pipeline: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsQuery, IsQuery, [id: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsRenderbuffer, IsRenderbuffer, [renderbuffer: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsSampler, IsSampler, [sampler: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsShader, IsShader, [shader: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsSync, IsSync, [sync: GLsync, ] -> GLboolean);
  mk_wrapper_method!(glIsTexture, IsTexture, [texture: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsTransformFeedback, IsTransformFeedback, [id: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glIsVertexArray, IsVertexArray, [array: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glLineWidth, LineWidth, [width: GLfloat, ] -> ());
  mk_wrapper_method!(glLinkProgram, LinkProgram, [program: GLuint, ] -> ());
  mk_wrapper_method!(glLogicOp, LogicOp, [opcode: GLenum, ] -> ());
  mk_wrapper_method!(glMapBuffer, MapBuffer, [target: GLenum, access: GLenum, ] -> *mut c_void);
  mk_wrapper_method!(glMapBufferRange, MapBufferRange, [target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield, ] -> *mut c_void);
  mk_wrapper_method!(glMapNamedBuffer, MapNamedBuffer, [buffer: GLuint, access: GLenum, ] -> *mut c_void);
  mk_wrapper_method!(glMapNamedBufferRange, MapNamedBufferRange, [buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield, ] -> *mut c_void);
  mk_wrapper_method!(glMemoryBarrier, MemoryBarrier, [barriers: GLbitfield, ] -> ());
  mk_wrapper_method!(glMemoryBarrierByRegion, MemoryBarrierByRegion, [barriers: GLbitfield, ] -> ());
  mk_wrapper_method!(glMinSampleShading, MinSampleShading, [value: GLfloat, ] -> ());
  mk_wrapper_method!(glMultiDrawArrays, MultiDrawArrays, [mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei, ] -> ());
  mk_wrapper_method!(glMultiDrawArraysIndirect, MultiDrawArraysIndirect, [mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei, ] -> ());
  mk_wrapper_method!(glMultiDrawArraysIndirectCount, MultiDrawArraysIndirectCount, [mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei, ] -> ());
  mk_wrapper_method!(glMultiDrawElements, MultiDrawElements, [mode: GLenum, count: *const GLsizei, ty: GLenum, indices: *const *const c_void, drawcount: GLsizei, ] -> ());
  mk_wrapper_method!(glMultiDrawElementsBaseVertex, MultiDrawElementsBaseVertex, [mode: GLenum, count: *const GLsizei, ty: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint, ] -> ());
  mk_wrapper_method!(glMultiDrawElementsIndirect, MultiDrawElementsIndirect, [mode: GLenum, ty: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei, ] -> ());
  mk_wrapper_method!(glMultiDrawElementsIndirectCount, MultiDrawElementsIndirectCount, [mode: GLenum, ty: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP1ui, MultiTexCoordP1ui, [texture: GLenum, ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP1uiv, MultiTexCoordP1uiv, [texture: GLenum, ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP2ui, MultiTexCoordP2ui, [texture: GLenum, ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP2uiv, MultiTexCoordP2uiv, [texture: GLenum, ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP3ui, MultiTexCoordP3ui, [texture: GLenum, ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP3uiv, MultiTexCoordP3uiv, [texture: GLenum, ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP4ui, MultiTexCoordP4ui, [texture: GLenum, ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glMultiTexCoordP4uiv, MultiTexCoordP4uiv, [texture: GLenum, ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glNamedBufferData, NamedBufferData, [buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum, ] -> ());
  mk_wrapper_method!(glNamedBufferStorage, NamedBufferStorage, [buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield, ] -> ());
  mk_wrapper_method!(glNamedBufferSubData, NamedBufferSubData, [buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void, ] -> ());
  mk_wrapper_method!(glNamedFramebufferDrawBuffer, NamedFramebufferDrawBuffer, [framebuffer: GLuint, buf: GLenum, ] -> ());
  mk_wrapper_method!(glNamedFramebufferDrawBuffers, NamedFramebufferDrawBuffers, [framebuffer: GLuint, n: GLsizei, bufs: *const GLenum, ] -> ());
  mk_wrapper_method!(glNamedFramebufferParameteri, NamedFramebufferParameteri, [framebuffer: GLuint, pname: GLenum, param: GLint, ] -> ());
  mk_wrapper_method!(glNamedFramebufferReadBuffer, NamedFramebufferReadBuffer, [framebuffer: GLuint, src: GLenum, ] -> ());
  mk_wrapper_method!(glNamedFramebufferRenderbuffer, NamedFramebufferRenderbuffer, [framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint, ] -> ());
  mk_wrapper_method!(glNamedFramebufferTexture, NamedFramebufferTexture, [framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, ] -> ());
  mk_wrapper_method!(glNamedFramebufferTextureLayer, NamedFramebufferTextureLayer, [framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint, ] -> ());
  mk_wrapper_method!(glNamedRenderbufferStorage, NamedRenderbufferStorage, [renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glNamedRenderbufferStorageMultisample, NamedRenderbufferStorageMultisample, [renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glNormalP3ui, NormalP3ui, [ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glNormalP3uiv, NormalP3uiv, [ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glObjectLabel, ObjectLabel, [identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar, ] -> ());
  mk_wrapper_method!(glObjectLabelKHR, ObjectLabelKHR, [identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar, ] -> ());
  mk_wrapper_method!(glObjectPtrLabel, ObjectPtrLabel, [ptr: *const c_void, length: GLsizei, label: *const GLchar, ] -> ());
  mk_wrapper_method!(glObjectPtrLabelKHR, ObjectPtrLabelKHR, [ptr: *const c_void, length: GLsizei, label: *const GLchar, ] -> ());
  mk_wrapper_method!(glPatchParameterfv, PatchParameterfv, [pname: GLenum, values: *const GLfloat, ] -> ());
  mk_wrapper_method!(glPatchParameteri, PatchParameteri, [pname: GLenum, value: GLint, ] -> ());
  mk_wrapper_method!(glPauseTransformFeedback, PauseTransformFeedback, [] -> ());
  mk_wrapper_method!(glPixelStoref, PixelStoref, [pname: GLenum, param: GLfloat, ] -> ());
  mk_wrapper_method!(glPixelStorei, PixelStorei, [pname: GLenum, param: GLint, ] -> ());
  mk_wrapper_method!(glPointParameterf, PointParameterf, [pname: GLenum, param: GLfloat, ] -> ());
  mk_wrapper_method!(glPointParameterfv, PointParameterfv, [pname: GLenum, params: *const GLfloat, ] -> ());
  mk_wrapper_method!(glPointParameteri, PointParameteri, [pname: GLenum, param: GLint, ] -> ());
  mk_wrapper_method!(glPointParameteriv, PointParameteriv, [pname: GLenum, params: *const GLint, ] -> ());
  mk_wrapper_method!(glPointSize, PointSize, [size: GLfloat, ] -> ());
  mk_wrapper_method!(glPolygonMode, PolygonMode, [face: GLenum, mode: GLenum, ] -> ());
  mk_wrapper_method!(glPolygonOffset, PolygonOffset, [factor: GLfloat, units: GLfloat, ] -> ());
  mk_wrapper_method!(glPolygonOffsetClamp, PolygonOffsetClamp, [factor: GLfloat, units: GLfloat, clamp: GLfloat, ] -> ());
  mk_wrapper_method!(glPopDebugGroup, PopDebugGroup, [] -> ());
  mk_wrapper_method!(glPopDebugGroupKHR, PopDebugGroupKHR, [] -> ());
  mk_wrapper_method!(glPrimitiveBoundingBox, PrimitiveBoundingBox, [minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat, ] -> ());
  mk_wrapper_method!(glPrimitiveRestartIndex, PrimitiveRestartIndex, [index: GLuint, ] -> ());
  mk_wrapper_method!(glProgramBinary, ProgramBinary, [program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei, ] -> ());
  mk_wrapper_method!(glProgramParameteri, ProgramParameteri, [program: GLuint, pname: GLenum, value: GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform1d, ProgramUniform1d, [program: GLuint, location: GLint, v0: GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform1dv, ProgramUniform1dv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform1f, ProgramUniform1f, [program: GLuint, location: GLint, v0: GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform1fv, ProgramUniform1fv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform1i, ProgramUniform1i, [program: GLuint, location: GLint, v0: GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform1iv, ProgramUniform1iv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform1ui, ProgramUniform1ui, [program: GLuint, location: GLint, v0: GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniform1uiv, ProgramUniform1uiv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniform2d, ProgramUniform2d, [program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform2dv, ProgramUniform2dv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform2f, ProgramUniform2f, [program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform2fv, ProgramUniform2fv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform2i, ProgramUniform2i, [program: GLuint, location: GLint, v0: GLint, v1: GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform2iv, ProgramUniform2iv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform2ui, ProgramUniform2ui, [program: GLuint, location: GLint, v0: GLuint, v1: GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniform2uiv, ProgramUniform2uiv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniform3d, ProgramUniform3d, [program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform3dv, ProgramUniform3dv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform3f, ProgramUniform3f, [program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform3fv, ProgramUniform3fv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform3i, ProgramUniform3i, [program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform3iv, ProgramUniform3iv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform3ui, ProgramUniform3ui, [program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniform3uiv, ProgramUniform3uiv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniform4d, ProgramUniform4d, [program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform4dv, ProgramUniform4dv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniform4f, ProgramUniform4f, [program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform4fv, ProgramUniform4fv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniform4i, ProgramUniform4i, [program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform4iv, ProgramUniform4iv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glProgramUniform4ui, ProgramUniform4ui, [program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniform4uiv, ProgramUniform4uiv, [program: GLuint, location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix2dv, ProgramUniformMatrix2dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix2fv, ProgramUniformMatrix2fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix2x3dv, ProgramUniformMatrix2x3dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix2x3fv, ProgramUniformMatrix2x3fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix2x4dv, ProgramUniformMatrix2x4dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix2x4fv, ProgramUniformMatrix2x4fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix3dv, ProgramUniformMatrix3dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix3fv, ProgramUniformMatrix3fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix3x2dv, ProgramUniformMatrix3x2dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix3x2fv, ProgramUniformMatrix3x2fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix3x4dv, ProgramUniformMatrix3x4dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix3x4fv, ProgramUniformMatrix3x4fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix4dv, ProgramUniformMatrix4dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix4fv, ProgramUniformMatrix4fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix4x2dv, ProgramUniformMatrix4x2dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix4x2fv, ProgramUniformMatrix4x2fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix4x3dv, ProgramUniformMatrix4x3dv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glProgramUniformMatrix4x3fv, ProgramUniformMatrix4x3fv, [program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glProvokingVertex, ProvokingVertex, [mode: GLenum, ] -> ());
  mk_wrapper_method!(glPushDebugGroup, PushDebugGroup, [source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar, ] -> ());
  mk_wrapper_method!(glPushDebugGroupKHR, PushDebugGroupKHR, [source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar, ] -> ());
  mk_wrapper_method!(glQueryCounter, QueryCounter, [id: GLuint, target: GLenum, ] -> ());
  mk_wrapper_method!(glReadBuffer, ReadBuffer, [src: GLenum, ] -> ());
  mk_wrapper_method!(glReadPixels, ReadPixels, [x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, ty: GLenum, pixels: *mut c_void, ] -> ());
  mk_wrapper_method!(glReadnPixels, ReadnPixels, [x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, ty: GLenum, bufSize: GLsizei, data: *mut c_void, ] -> ());
  mk_wrapper_method!(glReleaseShaderCompiler, ReleaseShaderCompiler, [] -> ());
  mk_wrapper_method!(glRenderbufferStorage, RenderbufferStorage, [target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glRenderbufferStorageMultisample, RenderbufferStorageMultisample, [target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glResumeTransformFeedback, ResumeTransformFeedback, [] -> ());
  mk_wrapper_method!(glSampleCoverage, SampleCoverage, [value: GLfloat, invert: GLboolean, ] -> ());
  mk_wrapper_method!(glSampleMaski, SampleMaski, [maskNumber: GLuint, mask: GLbitfield, ] -> ());
  mk_wrapper_method!(glSamplerParameterIiv, SamplerParameterIiv, [sampler: GLuint, pname: GLenum, param: *const GLint, ] -> ());
  mk_wrapper_method!(glSamplerParameterIuiv, SamplerParameterIuiv, [sampler: GLuint, pname: GLenum, param: *const GLuint, ] -> ());
  mk_wrapper_method!(glSamplerParameterf, SamplerParameterf, [sampler: GLuint, pname: GLenum, param: GLfloat, ] -> ());
  mk_wrapper_method!(glSamplerParameterfv, SamplerParameterfv, [sampler: GLuint, pname: GLenum, param: *const GLfloat, ] -> ());
  mk_wrapper_method!(glSamplerParameteri, SamplerParameteri, [sampler: GLuint, pname: GLenum, param: GLint, ] -> ());
  mk_wrapper_method!(glSamplerParameteriv, SamplerParameteriv, [sampler: GLuint, pname: GLenum, param: *const GLint, ] -> ());
  mk_wrapper_method!(glScissor, Scissor, [x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glScissorArrayv, ScissorArrayv, [first: GLuint, count: GLsizei, v: *const GLint, ] -> ());
  mk_wrapper_method!(glScissorIndexed, ScissorIndexed, [index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glScissorIndexedv, ScissorIndexedv, [index: GLuint, v: *const GLint, ] -> ());
  mk_wrapper_method!(glSecondaryColorP3ui, SecondaryColorP3ui, [ty: GLenum, color: GLuint, ] -> ());
  mk_wrapper_method!(glSecondaryColorP3uiv, SecondaryColorP3uiv, [ty: GLenum, color: *const GLuint, ] -> ());
  mk_wrapper_method!(glShaderBinary, ShaderBinary, [count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei, ] -> ());
  mk_wrapper_method!(glShaderSource, ShaderSource, [shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint, ] -> ());
  mk_wrapper_method!(glShaderStorageBlockBinding, ShaderStorageBlockBinding, [program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint, ] -> ());
  mk_wrapper_method!(glSpecializeShader, SpecializeShader, [shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint, ] -> ());
  mk_wrapper_method!(glStencilFunc, StencilFunc, [func: GLenum, reference: GLint, mask: GLuint, ] -> ());
  mk_wrapper_method!(glStencilFuncSeparate, StencilFuncSeparate, [face: GLenum, func: GLenum, reference: GLint, mask: GLuint, ] -> ());
  mk_wrapper_method!(glStencilMask, StencilMask, [mask: GLuint, ] -> ());
  mk_wrapper_method!(glStencilMaskSeparate, StencilMaskSeparate, [face: GLenum, mask: GLuint, ] -> ());
  mk_wrapper_method!(glStencilOp, StencilOp, [fail: GLenum, zfail: GLenum, zpass: GLenum, ] -> ());
  mk_wrapper_method!(glStencilOpSeparate, StencilOpSeparate, [face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum, ] -> ());
  mk_wrapper_method!(glTexBuffer, TexBuffer, [target: GLenum, internalformat: GLenum, buffer: GLuint, ] -> ());
  mk_wrapper_method!(glTexBufferRange, TexBufferRange, [target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glTexCoordP1ui, TexCoordP1ui, [ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glTexCoordP1uiv, TexCoordP1uiv, [ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glTexCoordP2ui, TexCoordP2ui, [ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glTexCoordP2uiv, TexCoordP2uiv, [ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glTexCoordP3ui, TexCoordP3ui, [ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glTexCoordP3uiv, TexCoordP3uiv, [ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glTexCoordP4ui, TexCoordP4ui, [ty: GLenum, coords: GLuint, ] -> ());
  mk_wrapper_method!(glTexCoordP4uiv, TexCoordP4uiv, [ty: GLenum, coords: *const GLuint, ] -> ());
  mk_wrapper_method!(glTexImage1D, TexImage1D, [target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTexImage2D, TexImage2D, [target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTexImage2DMultisample, TexImage2DMultisample, [target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean, ] -> ());
  mk_wrapper_method!(glTexImage3D, TexImage3D, [target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTexImage3DMultisample, TexImage3DMultisample, [target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean, ] -> ());
  mk_wrapper_method!(glTexParameterIiv, TexParameterIiv, [target: GLenum, pname: GLenum, params: *const GLint, ] -> ());
  mk_wrapper_method!(glTexParameterIuiv, TexParameterIuiv, [target: GLenum, pname: GLenum, params: *const GLuint, ] -> ());
  mk_wrapper_method!(glTexParameterf, TexParameterf, [target: GLenum, pname: GLenum, param: GLfloat, ] -> ());
  mk_wrapper_method!(glTexParameterfv, TexParameterfv, [target: GLenum, pname: GLenum, params: *const GLfloat, ] -> ());
  mk_wrapper_method!(glTexParameteri, TexParameteri, [target: GLenum, pname: GLenum, param: GLint, ] -> ());
  mk_wrapper_method!(glTexParameteriv, TexParameteriv, [target: GLenum, pname: GLenum, params: *const GLint, ] -> ());
  mk_wrapper_method!(glTexStorage1D, TexStorage1D, [target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, ] -> ());
  mk_wrapper_method!(glTexStorage2D, TexStorage2D, [target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glTexStorage2DMultisample, TexStorage2DMultisample, [target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean, ] -> ());
  mk_wrapper_method!(glTexStorage3D, TexStorage3D, [target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, ] -> ());
  mk_wrapper_method!(glTexStorage3DMultisample, TexStorage3DMultisample, [target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean, ] -> ());
  mk_wrapper_method!(glTexSubImage1D, TexSubImage1D, [target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTexSubImage2D, TexSubImage2D, [target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTexSubImage3D, TexSubImage3D, [target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTextureBarrier, TextureBarrier, [] -> ());
  mk_wrapper_method!(glTextureBuffer, TextureBuffer, [texture: GLuint, internalformat: GLenum, buffer: GLuint, ] -> ());
  mk_wrapper_method!(glTextureBufferRange, TextureBufferRange, [texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glTextureParameterIiv, TextureParameterIiv, [texture: GLuint, pname: GLenum, params: *const GLint, ] -> ());
  mk_wrapper_method!(glTextureParameterIuiv, TextureParameterIuiv, [texture: GLuint, pname: GLenum, params: *const GLuint, ] -> ());
  mk_wrapper_method!(glTextureParameterf, TextureParameterf, [texture: GLuint, pname: GLenum, param: GLfloat, ] -> ());
  mk_wrapper_method!(glTextureParameterfv, TextureParameterfv, [texture: GLuint, pname: GLenum, param: *const GLfloat, ] -> ());
  mk_wrapper_method!(glTextureParameteri, TextureParameteri, [texture: GLuint, pname: GLenum, param: GLint, ] -> ());
  mk_wrapper_method!(glTextureParameteriv, TextureParameteriv, [texture: GLuint, pname: GLenum, param: *const GLint, ] -> ());
  mk_wrapper_method!(glTextureStorage1D, TextureStorage1D, [texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, ] -> ());
  mk_wrapper_method!(glTextureStorage2D, TextureStorage2D, [texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glTextureStorage2DMultisample, TextureStorage2DMultisample, [texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean, ] -> ());
  mk_wrapper_method!(glTextureStorage3D, TextureStorage3D, [texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, ] -> ());
  mk_wrapper_method!(glTextureStorage3DMultisample, TextureStorage3DMultisample, [texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean, ] -> ());
  mk_wrapper_method!(glTextureSubImage1D, TextureSubImage1D, [texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTextureSubImage2D, TextureSubImage2D, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTextureSubImage3D, TextureSubImage3D, [texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, ty: GLenum, pixels: *const c_void, ] -> ());
  mk_wrapper_method!(glTextureView, TextureView, [texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint, ] -> ());
  mk_wrapper_method!(glTransformFeedbackBufferBase, TransformFeedbackBufferBase, [xfb: GLuint, index: GLuint, buffer: GLuint, ] -> ());
  mk_wrapper_method!(glTransformFeedbackBufferRange, TransformFeedbackBufferRange, [xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, ] -> ());
  mk_wrapper_method!(glTransformFeedbackVaryings, TransformFeedbackVaryings, [program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum, ] -> ());
  mk_wrapper_method!(glUniform1d, Uniform1d, [location: GLint, x: GLdouble, ] -> ());
  mk_wrapper_method!(glUniform1dv, Uniform1dv, [location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniform1f, Uniform1f, [location: GLint, v0: GLfloat, ] -> ());
  mk_wrapper_method!(glUniform1fv, Uniform1fv, [location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniform1i, Uniform1i, [location: GLint, v0: GLint, ] -> ());
  mk_wrapper_method!(glUniform1iv, Uniform1iv, [location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glUniform1ui, Uniform1ui, [location: GLint, v0: GLuint, ] -> ());
  mk_wrapper_method!(glUniform1uiv, Uniform1uiv, [location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glUniform2d, Uniform2d, [location: GLint, x: GLdouble, y: GLdouble, ] -> ());
  mk_wrapper_method!(glUniform2dv, Uniform2dv, [location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniform2f, Uniform2f, [location: GLint, v0: GLfloat, v1: GLfloat, ] -> ());
  mk_wrapper_method!(glUniform2fv, Uniform2fv, [location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniform2i, Uniform2i, [location: GLint, v0: GLint, v1: GLint, ] -> ());
  mk_wrapper_method!(glUniform2iv, Uniform2iv, [location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glUniform2ui, Uniform2ui, [location: GLint, v0: GLuint, v1: GLuint, ] -> ());
  mk_wrapper_method!(glUniform2uiv, Uniform2uiv, [location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glUniform3d, Uniform3d, [location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, ] -> ());
  mk_wrapper_method!(glUniform3dv, Uniform3dv, [location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniform3f, Uniform3f, [location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, ] -> ());
  mk_wrapper_method!(glUniform3fv, Uniform3fv, [location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniform3i, Uniform3i, [location: GLint, v0: GLint, v1: GLint, v2: GLint, ] -> ());
  mk_wrapper_method!(glUniform3iv, Uniform3iv, [location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glUniform3ui, Uniform3ui, [location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, ] -> ());
  mk_wrapper_method!(glUniform3uiv, Uniform3uiv, [location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glUniform4d, Uniform4d, [location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble, ] -> ());
  mk_wrapper_method!(glUniform4dv, Uniform4dv, [location: GLint, count: GLsizei, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniform4f, Uniform4f, [location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat, ] -> ());
  mk_wrapper_method!(glUniform4fv, Uniform4fv, [location: GLint, count: GLsizei, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniform4i, Uniform4i, [location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint, ] -> ());
  mk_wrapper_method!(glUniform4iv, Uniform4iv, [location: GLint, count: GLsizei, value: *const GLint, ] -> ());
  mk_wrapper_method!(glUniform4ui, Uniform4ui, [location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint, ] -> ());
  mk_wrapper_method!(glUniform4uiv, Uniform4uiv, [location: GLint, count: GLsizei, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glUniformBlockBinding, UniformBlockBinding, [program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint, ] -> ());
  mk_wrapper_method!(glUniformMatrix2dv, UniformMatrix2dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix2fv, UniformMatrix2fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix2x3dv, UniformMatrix2x3dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix2x3fv, UniformMatrix2x3fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix2x4dv, UniformMatrix2x4dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix2x4fv, UniformMatrix2x4fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix3dv, UniformMatrix3dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix3fv, UniformMatrix3fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix3x2dv, UniformMatrix3x2dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix3x2fv, UniformMatrix3x2fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix3x4dv, UniformMatrix3x4dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix3x4fv, UniformMatrix3x4fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix4dv, UniformMatrix4dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix4fv, UniformMatrix4fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix4x2dv, UniformMatrix4x2dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix4x2fv, UniformMatrix4x2fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformMatrix4x3dv, UniformMatrix4x3dv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble, ] -> ());
  mk_wrapper_method!(glUniformMatrix4x3fv, UniformMatrix4x3fv, [location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat, ] -> ());
  mk_wrapper_method!(glUniformSubroutinesuiv, UniformSubroutinesuiv, [shadertype: GLenum, count: GLsizei, indices: *const GLuint, ] -> ());
  mk_wrapper_method!(glUnmapBuffer, UnmapBuffer, [target: GLenum, ] -> GLboolean);
  mk_wrapper_method!(glUnmapNamedBuffer, UnmapNamedBuffer, [buffer: GLuint, ] -> GLboolean);
  mk_wrapper_method!(glUseProgram, UseProgram, [program: GLuint, ] -> ());
  mk_wrapper_method!(glUseProgramStages, UseProgramStages, [pipeline: GLuint, stages: GLbitfield, program: GLuint, ] -> ());
  mk_wrapper_method!(glValidateProgram, ValidateProgram, [program: GLuint, ] -> ());
  mk_wrapper_method!(glValidateProgramPipeline, ValidateProgramPipeline, [pipeline: GLuint, ] -> ());
  mk_wrapper_method!(glVertexArrayAttribBinding, VertexArrayAttribBinding, [vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint, ] -> ());
  mk_wrapper_method!(glVertexArrayAttribFormat, VertexArrayAttribFormat, [vaobj: GLuint, attribindex: GLuint, size: GLint, ty: GLenum, normalized: GLboolean, relativeoffset: GLuint, ] -> ());
  mk_wrapper_method!(glVertexArrayAttribIFormat, VertexArrayAttribIFormat, [vaobj: GLuint, attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint, ] -> ());
  mk_wrapper_method!(glVertexArrayAttribLFormat, VertexArrayAttribLFormat, [vaobj: GLuint, attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint, ] -> ());
  mk_wrapper_method!(glVertexArrayBindingDivisor, VertexArrayBindingDivisor, [vaobj: GLuint, bindingindex: GLuint, divisor: GLuint, ] -> ());
  mk_wrapper_method!(glVertexArrayElementBuffer, VertexArrayElementBuffer, [vaobj: GLuint, buffer: GLuint, ] -> ());
  mk_wrapper_method!(glVertexArrayVertexBuffer, VertexArrayVertexBuffer, [vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei, ] -> ());
  mk_wrapper_method!(glVertexArrayVertexBuffers, VertexArrayVertexBuffers, [vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei, ] -> ());
  mk_wrapper_method!(glVertexAttrib1d, VertexAttrib1d, [index: GLuint, x: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib1dv, VertexAttrib1dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib1f, VertexAttrib1f, [index: GLuint, x: GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib1fv, VertexAttrib1fv, [index: GLuint, v: *const GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib1s, VertexAttrib1s, [index: GLuint, x: GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib1sv, VertexAttrib1sv, [index: GLuint, v: *const GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib2d, VertexAttrib2d, [index: GLuint, x: GLdouble, y: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib2dv, VertexAttrib2dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib2f, VertexAttrib2f, [index: GLuint, x: GLfloat, y: GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib2fv, VertexAttrib2fv, [index: GLuint, v: *const GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib2s, VertexAttrib2s, [index: GLuint, x: GLshort, y: GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib2sv, VertexAttrib2sv, [index: GLuint, v: *const GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib3d, VertexAttrib3d, [index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib3dv, VertexAttrib3dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib3f, VertexAttrib3f, [index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib3fv, VertexAttrib3fv, [index: GLuint, v: *const GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib3s, VertexAttrib3s, [index: GLuint, x: GLshort, y: GLshort, z: GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib3sv, VertexAttrib3sv, [index: GLuint, v: *const GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib4Nbv, VertexAttrib4Nbv, [index: GLuint, v: *const GLbyte, ] -> ());
  mk_wrapper_method!(glVertexAttrib4Niv, VertexAttrib4Niv, [index: GLuint, v: *const GLint, ] -> ());
  mk_wrapper_method!(glVertexAttrib4Nsv, VertexAttrib4Nsv, [index: GLuint, v: *const GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib4Nub, VertexAttrib4Nub, [index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte, ] -> ());
  mk_wrapper_method!(glVertexAttrib4Nubv, VertexAttrib4Nubv, [index: GLuint, v: *const GLubyte, ] -> ());
  mk_wrapper_method!(glVertexAttrib4Nuiv, VertexAttrib4Nuiv, [index: GLuint, v: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttrib4Nusv, VertexAttrib4Nusv, [index: GLuint, v: *const GLushort, ] -> ());
  mk_wrapper_method!(glVertexAttrib4bv, VertexAttrib4bv, [index: GLuint, v: *const GLbyte, ] -> ());
  mk_wrapper_method!(glVertexAttrib4d, VertexAttrib4d, [index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib4dv, VertexAttrib4dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttrib4f, VertexAttrib4f, [index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib4fv, VertexAttrib4fv, [index: GLuint, v: *const GLfloat, ] -> ());
  mk_wrapper_method!(glVertexAttrib4iv, VertexAttrib4iv, [index: GLuint, v: *const GLint, ] -> ());
  mk_wrapper_method!(glVertexAttrib4s, VertexAttrib4s, [index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib4sv, VertexAttrib4sv, [index: GLuint, v: *const GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttrib4ubv, VertexAttrib4ubv, [index: GLuint, v: *const GLubyte, ] -> ());
  mk_wrapper_method!(glVertexAttrib4uiv, VertexAttrib4uiv, [index: GLuint, v: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttrib4usv, VertexAttrib4usv, [index: GLuint, v: *const GLushort, ] -> ());
  mk_wrapper_method!(glVertexAttribBinding, VertexAttribBinding, [attribindex: GLuint, bindingindex: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribDivisor, VertexAttribDivisor, [index: GLuint, divisor: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribFormat, VertexAttribFormat, [attribindex: GLuint, size: GLint, ty: GLenum, normalized: GLboolean, relativeoffset: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI1i, VertexAttribI1i, [index: GLuint, x: GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI1iv, VertexAttribI1iv, [index: GLuint, v: *const GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI1ui, VertexAttribI1ui, [index: GLuint, x: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI1uiv, VertexAttribI1uiv, [index: GLuint, v: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI2i, VertexAttribI2i, [index: GLuint, x: GLint, y: GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI2iv, VertexAttribI2iv, [index: GLuint, v: *const GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI2ui, VertexAttribI2ui, [index: GLuint, x: GLuint, y: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI2uiv, VertexAttribI2uiv, [index: GLuint, v: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI3i, VertexAttribI3i, [index: GLuint, x: GLint, y: GLint, z: GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI3iv, VertexAttribI3iv, [index: GLuint, v: *const GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI3ui, VertexAttribI3ui, [index: GLuint, x: GLuint, y: GLuint, z: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI3uiv, VertexAttribI3uiv, [index: GLuint, v: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI4bv, VertexAttribI4bv, [index: GLuint, v: *const GLbyte, ] -> ());
  mk_wrapper_method!(glVertexAttribI4i, VertexAttribI4i, [index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI4iv, VertexAttribI4iv, [index: GLuint, v: *const GLint, ] -> ());
  mk_wrapper_method!(glVertexAttribI4sv, VertexAttribI4sv, [index: GLuint, v: *const GLshort, ] -> ());
  mk_wrapper_method!(glVertexAttribI4ubv, VertexAttribI4ubv, [index: GLuint, v: *const GLubyte, ] -> ());
  mk_wrapper_method!(glVertexAttribI4ui, VertexAttribI4ui, [index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI4uiv, VertexAttribI4uiv, [index: GLuint, v: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribI4usv, VertexAttribI4usv, [index: GLuint, v: *const GLushort, ] -> ());
  mk_wrapper_method!(glVertexAttribIFormat, VertexAttribIFormat, [attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribIPointer, VertexAttribIPointer, [index: GLuint, size: GLint, ty: GLenum, stride: GLsizei, pointer: *const c_void, ] -> ());
  mk_wrapper_method!(glVertexAttribL1d, VertexAttribL1d, [index: GLuint, x: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribL1dv, VertexAttribL1dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribL2d, VertexAttribL2d, [index: GLuint, x: GLdouble, y: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribL2dv, VertexAttribL2dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribL3d, VertexAttribL3d, [index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribL3dv, VertexAttribL3dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribL4d, VertexAttribL4d, [index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribL4dv, VertexAttribL4dv, [index: GLuint, v: *const GLdouble, ] -> ());
  mk_wrapper_method!(glVertexAttribLFormat, VertexAttribLFormat, [attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribLPointer, VertexAttribLPointer, [index: GLuint, size: GLint, ty: GLenum, stride: GLsizei, pointer: *const c_void, ] -> ());
  mk_wrapper_method!(glVertexAttribP1ui, VertexAttribP1ui, [index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribP1uiv, VertexAttribP1uiv, [index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribP2ui, VertexAttribP2ui, [index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribP2uiv, VertexAttribP2uiv, [index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribP3ui, VertexAttribP3ui, [index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribP3uiv, VertexAttribP3uiv, [index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribP4ui, VertexAttribP4ui, [index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribP4uiv, VertexAttribP4uiv, [index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexAttribPointer, VertexAttribPointer, [index: GLuint, size: GLint, ty: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void, ] -> ());
  mk_wrapper_method!(glVertexBindingDivisor, VertexBindingDivisor, [bindingindex: GLuint, divisor: GLuint, ] -> ());
  mk_wrapper_method!(glVertexP2ui, VertexP2ui, [ty: GLenum, value: GLuint, ] -> ());
  mk_wrapper_method!(glVertexP2uiv, VertexP2uiv, [ty: GLenum, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexP3ui, VertexP3ui, [ty: GLenum, value: GLuint, ] -> ());
  mk_wrapper_method!(glVertexP3uiv, VertexP3uiv, [ty: GLenum, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glVertexP4ui, VertexP4ui, [ty: GLenum, value: GLuint, ] -> ());
  mk_wrapper_method!(glVertexP4uiv, VertexP4uiv, [ty: GLenum, value: *const GLuint, ] -> ());
  mk_wrapper_method!(glViewport, Viewport, [x: GLint, y: GLint, width: GLsizei, height: GLsizei, ] -> ());
  mk_wrapper_method!(glViewportArrayv, ViewportArrayv, [first: GLuint, count: GLsizei, v: *const GLfloat, ] -> ());
  mk_wrapper_method!(glViewportIndexedf, ViewportIndexedf, [index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat, ] -> ());
  mk_wrapper_method!(glViewportIndexedfv, ViewportIndexedfv, [index: GLuint, v: *const GLfloat, ] -> ());
  mk_wrapper_method!(glWaitSync, WaitSync, [sync: GLsync, flags: GLbitfield, timeout: GLuint64, ] -> ());
}
use fn_ty_aliases::*;
mod fn_ty_aliases {
  use super::*;
  pub type glActiveShaderProgram_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);
  pub type glActiveTexture_t = unsafe extern "system" fn(texture: GLenum);
  pub type glAttachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);
  pub type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: GLenum);
  pub type glBeginQuery_t = unsafe extern "system" fn(target: GLenum, id: GLuint);
  pub type glBeginQueryIndexed_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, id: GLuint);
  pub type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: GLenum);
  pub type glBindAttribLocation_t =
    unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);
  pub type glBindBuffer_t = unsafe extern "system" fn(target: GLenum, buffer: GLuint);
  pub type glBindBufferBase_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint);
  pub type glBindBufferRange_t = unsafe extern "system" fn(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
  );
  pub type glBindBuffersBase_t = unsafe extern "system" fn(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
  );
  pub type glBindBuffersRange_t = unsafe extern "system" fn(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    sizes: *const GLsizeiptr,
  );
  pub type glBindFragDataLocation_t =
    unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);
  pub type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(
    program: GLuint,
    colorNumber: GLuint,
    index: GLuint,
    name: *const GLchar,
  );
  pub type glBindFramebuffer_t = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);
  pub type glBindImageTexture_t = unsafe extern "system" fn(
    unit: GLuint,
    texture: GLuint,
    level: GLint,
    layered: GLboolean,
    layer: GLint,
    access: GLenum,
    format: GLenum,
  );
  pub type glBindImageTextures_t =
    unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
  pub type glBindProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);
  pub type glBindRenderbuffer_t = unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);
  pub type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);
  pub type glBindSamplers_t =
    unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);
  pub type glBindTexture_t = unsafe extern "system" fn(target: GLenum, texture: GLuint);
  pub type glBindTextureUnit_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint);
  pub type glBindTextures_t =
    unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
  pub type glBindTransformFeedback_t = unsafe extern "system" fn(target: GLenum, id: GLuint);
  pub type glBindVertexArray_t = unsafe extern "system" fn(array: GLuint);
  pub type glBindVertexBuffer_t = unsafe extern "system" fn(
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
  );
  pub type glBindVertexBuffers_t = unsafe extern "system" fn(
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
  );
  pub type glBlendBarrier_t = unsafe extern "system" fn();
  pub type glBlendColor_t =
    unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
  pub type glBlendEquation_t = unsafe extern "system" fn(mode: GLenum);
  pub type glBlendEquationSeparate_t =
    unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum);
  pub type glBlendEquationSeparatei_t =
    unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
  pub type glBlendEquationi_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);
  pub type glBlendFunc_t = unsafe extern "system" fn(sfactor: GLenum, dfactor: GLenum);
  pub type glBlendFuncSeparate_t = unsafe extern "system" fn(
    sfactorRGB: GLenum,
    dfactorRGB: GLenum,
    sfactorAlpha: GLenum,
    dfactorAlpha: GLenum,
  );
  pub type glBlendFuncSeparatei_t = unsafe extern "system" fn(
    buf: GLuint,
    srcRGB: GLenum,
    dstRGB: GLenum,
    srcAlpha: GLenum,
    dstAlpha: GLenum,
  );
  pub type glBlendFunci_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);
  pub type glBlitFramebuffer_t = unsafe extern "system" fn(
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
  );
  pub type glBlitNamedFramebuffer_t = unsafe extern "system" fn(
    readFramebuffer: GLuint,
    drawFramebuffer: GLuint,
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
  );
  pub type glBufferData_t =
    unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
  pub type glBufferStorage_t = unsafe extern "system" fn(
    target: GLenum,
    size: GLsizeiptr,
    data: *const c_void,
    flags: GLbitfield,
  );
  pub type glBufferSubData_t = unsafe extern "system" fn(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
  );
  pub type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: GLenum) -> GLenum;
  pub type glCheckNamedFramebufferStatus_t =
    unsafe extern "system" fn(framebuffer: GLuint, target: GLenum) -> GLenum;
  pub type glClampColor_t = unsafe extern "system" fn(target: GLenum, clamp: GLenum);
  pub type glClear_t = unsafe extern "system" fn(mask: GLbitfield);
  pub type glClearBufferData_t = unsafe extern "system" fn(
    target: GLenum,
    internalformat: GLenum,
    format: GLenum,
    ty: GLenum,
    data: *const c_void,
  );
  pub type glClearBufferSubData_t = unsafe extern "system" fn(
    target: GLenum,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    ty: GLenum,
    data: *const c_void,
  );
  pub type glClearBufferfi_t =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
  pub type glClearBufferfv_t =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
  pub type glClearBufferiv_t =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
  pub type glClearBufferuiv_t =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
  pub type glClearColor_t =
    unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
  pub type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);
  pub type glClearDepthf_t = unsafe extern "system" fn(d: GLfloat);
  pub type glClearNamedBufferData_t = unsafe extern "system" fn(
    buffer: GLuint,
    internalformat: GLenum,
    format: GLenum,
    ty: GLenum,
    data: *const c_void,
  );
  pub type glClearNamedBufferSubData_t = unsafe extern "system" fn(
    buffer: GLuint,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    ty: GLenum,
    data: *const c_void,
  );
  pub type glClearNamedFramebufferfi_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    depth: GLfloat,
    stencil: GLint,
  );
  pub type glClearNamedFramebufferfv_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLfloat,
  );
  pub type glClearNamedFramebufferiv_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLint,
  );
  pub type glClearNamedFramebufferuiv_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLuint,
  );
  pub type glClearStencil_t = unsafe extern "system" fn(s: GLint);
  pub type glClearTexImage_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    ty: GLenum,
    data: *const c_void,
  );
  pub type glClearTexSubImage_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    ty: GLenum,
    data: *const c_void,
  );
  pub type glClientWaitSync_t =
    unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
  pub type glClipControl_t = unsafe extern "system" fn(origin: GLenum, depth: GLenum);
  pub type glColorMask_t =
    unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
  pub type glColorMaski_t = unsafe extern "system" fn(
    index: GLuint,
    r: GLboolean,
    g: GLboolean,
    b: GLboolean,
    a: GLboolean,
  );
  pub type glColorP3ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
  pub type glColorP3uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
  pub type glColorP4ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
  pub type glColorP4uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
  pub type glCompileShader_t = unsafe extern "system" fn(shader: GLuint);
  pub type glCompressedTexImage1D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTexImage2D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTexImage3D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTexSubImage1D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTexSubImage2D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTexSubImage3D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTextureSubImage1D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTextureSubImage2D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCompressedTextureSubImage3D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  pub type glCopyBufferSubData_t = unsafe extern "system" fn(
    readTarget: GLenum,
    writeTarget: GLenum,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
  );
  pub type glCopyImageSubData_t = unsafe extern "system" fn(
    srcName: GLuint,
    srcTarget: GLenum,
    srcLevel: GLint,
    srcX: GLint,
    srcY: GLint,
    srcZ: GLint,
    dstName: GLuint,
    dstTarget: GLenum,
    dstLevel: GLint,
    dstX: GLint,
    dstY: GLint,
    dstZ: GLint,
    srcWidth: GLsizei,
    srcHeight: GLsizei,
    srcDepth: GLsizei,
  );
  pub type glCopyNamedBufferSubData_t = unsafe extern "system" fn(
    readBuffer: GLuint,
    writeBuffer: GLuint,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
  );
  pub type glCopyTexImage1D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    border: GLint,
  );
  pub type glCopyTexImage2D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
  );
  pub type glCopyTexSubImage1D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
  );
  pub type glCopyTexSubImage2D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glCopyTexSubImage3D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glCopyTextureSubImage1D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
  );
  pub type glCopyTextureSubImage2D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glCopyTextureSubImage3D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glCreateBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
  pub type glCreateFramebuffers_t =
    unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);
  pub type glCreateProgram_t = unsafe extern "system" fn() -> GLuint;
  pub type glCreateProgramPipelines_t =
    unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);
  pub type glCreateQueries_t =
    unsafe extern "system" fn(target: GLenum, n: GLsizei, ids: *mut GLuint);
  pub type glCreateRenderbuffers_t =
    unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);
  pub type glCreateSamplers_t = unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint);
  pub type glCreateShader_t = unsafe extern "system" fn(ty: GLenum) -> GLuint;
  pub type glCreateShaderProgramv_t =
    unsafe extern "system" fn(ty: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint;
  pub type glCreateTextures_t =
    unsafe extern "system" fn(target: GLenum, n: GLsizei, textures: *mut GLuint);
  pub type glCreateTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
  pub type glCreateVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
  pub type glCullFace_t = unsafe extern "system" fn(mode: GLenum);
  pub type glDebugMessageCallback_t =
    unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const c_void);
  pub type glDebugMessageCallbackKHR_t =
    unsafe extern "system" fn(callback: GLDEBUGPROCKHR, userParam: *const c_void);
  pub type glDebugMessageControl_t = unsafe extern "system" fn(
    source: GLenum,
    ty: GLenum,
    severity: GLenum,
    count: GLsizei,
    ids: *const GLuint,
    enabled: GLboolean,
  );
  pub type glDebugMessageControlKHR_t = unsafe extern "system" fn(
    source: GLenum,
    ty: GLenum,
    severity: GLenum,
    count: GLsizei,
    ids: *const GLuint,
    enabled: GLboolean,
  );
  pub type glDebugMessageInsert_t = unsafe extern "system" fn(
    source: GLenum,
    ty: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    buf: *const GLchar,
  );
  pub type glDebugMessageInsertKHR_t = unsafe extern "system" fn(
    source: GLenum,
    ty: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    buf: *const GLchar,
  );
  pub type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);
  pub type glDeleteFramebuffers_t =
    unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);
  pub type glDeleteProgram_t = unsafe extern "system" fn(program: GLuint);
  pub type glDeleteProgramPipelines_t =
    unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);
  pub type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);
  pub type glDeleteRenderbuffers_t =
    unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);
  pub type glDeleteSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);
  pub type glDeleteShader_t = unsafe extern "system" fn(shader: GLuint);
  pub type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);
  pub type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);
  pub type glDeleteTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);
  pub type glDeleteVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);
  pub type glDepthFunc_t = unsafe extern "system" fn(func: GLenum);
  pub type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);
  pub type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);
  pub type glDepthRangeArrayv_t =
    unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);
  pub type glDepthRangeIndexed_t =
    unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);
  pub type glDepthRangef_t = unsafe extern "system" fn(n: GLfloat, f: GLfloat);
  pub type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);
  pub type glDisable_t = unsafe extern "system" fn(cap: GLenum);
  pub type glDisableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);
  pub type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);
  pub type glDisablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
  pub type glDispatchCompute_t =
    unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
  pub type glDispatchComputeIndirect_t = unsafe extern "system" fn(indirect: GLintptr);
  pub type glDrawArrays_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);
  pub type glDrawArraysIndirect_t =
    unsafe extern "system" fn(mode: GLenum, indirect: *const c_void);
  pub type glDrawArraysInstanced_t =
    unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
  pub type glDrawArraysInstancedBaseInstance_t = unsafe extern "system" fn(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
    baseinstance: GLuint,
  );
  pub type glDrawBuffer_t = unsafe extern "system" fn(buf: GLenum);
  pub type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);
  pub type glDrawElements_t =
    unsafe extern "system" fn(mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void);
  pub type glDrawElementsBaseVertex_t = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    ty: GLenum,
    indices: *const c_void,
    basevertex: GLint,
  );
  pub type glDrawElementsIndirect_t =
    unsafe extern "system" fn(mode: GLenum, ty: GLenum, indirect: *const c_void);
  pub type glDrawElementsInstanced_t = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    ty: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
  );
  pub type glDrawElementsInstancedBaseInstance_t = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    ty: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    baseinstance: GLuint,
  );
  pub type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    ty: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    basevertex: GLint,
  );
  pub type glDrawElementsInstancedBaseVertexBaseInstance_t = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    ty: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    basevertex: GLint,
    baseinstance: GLuint,
  );
  pub type glDrawRangeElements_t = unsafe extern "system" fn(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    ty: GLenum,
    indices: *const c_void,
  );
  pub type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    ty: GLenum,
    indices: *const c_void,
    basevertex: GLint,
  );
  pub type glDrawTransformFeedback_t = unsafe extern "system" fn(mode: GLenum, id: GLuint);
  pub type glDrawTransformFeedbackInstanced_t =
    unsafe extern "system" fn(mode: GLenum, id: GLuint, instancecount: GLsizei);
  pub type glDrawTransformFeedbackStream_t =
    unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint);
  pub type glDrawTransformFeedbackStreamInstanced_t =
    unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);
  pub type glEnable_t = unsafe extern "system" fn(cap: GLenum);
  pub type glEnableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);
  pub type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);
  pub type glEnablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
  pub type glEndConditionalRender_t = unsafe extern "system" fn();
  pub type glEndQuery_t = unsafe extern "system" fn(target: GLenum);
  pub type glEndQueryIndexed_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
  pub type glEndTransformFeedback_t = unsafe extern "system" fn();
  pub type glFenceSync_t =
    unsafe extern "system" fn(condition: GLenum, flags: GLbitfield) -> GLsync;
  pub type glFinish_t = unsafe extern "system" fn();
  pub type glFlush_t = unsafe extern "system" fn();
  pub type glFlushMappedBufferRange_t =
    unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr);
  pub type glFlushMappedNamedBufferRange_t =
    unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
  pub type glFramebufferParameteri_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
  pub type glFramebufferRenderbuffer_t = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
  );
  pub type glFramebufferTexture_t =
    unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
  pub type glFramebufferTexture1D_t = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
  );
  pub type glFramebufferTexture2D_t = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
  );
  pub type glFramebufferTexture3D_t = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
    zoffset: GLint,
  );
  pub type glFramebufferTextureLayer_t = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
  );
  pub type glFrontFace_t = unsafe extern "system" fn(mode: GLenum);
  pub type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
  pub type glGenFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);
  pub type glGenProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);
  pub type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
  pub type glGenRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);
  pub type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);
  pub type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);
  pub type glGenTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
  pub type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
  pub type glGenerateMipmap_t = unsafe extern "system" fn(target: GLenum);
  pub type glGenerateTextureMipmap_t = unsafe extern "system" fn(texture: GLuint);
  pub type glGetActiveAtomicCounterBufferiv_t = unsafe extern "system" fn(
    program: GLuint,
    bufferIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
  );
  pub type glGetActiveAttrib_t = unsafe extern "system" fn(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    ty: *mut GLenum,
    name: *mut GLchar,
  );
  pub type glGetActiveSubroutineName_t = unsafe extern "system" fn(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
  );
  pub type glGetActiveSubroutineUniformName_t = unsafe extern "system" fn(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
  );
  pub type glGetActiveSubroutineUniformiv_t = unsafe extern "system" fn(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    pname: GLenum,
    values: *mut GLint,
  );
  pub type glGetActiveUniform_t = unsafe extern "system" fn(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    ty: *mut GLenum,
    name: *mut GLchar,
  );
  pub type glGetActiveUniformBlockName_t = unsafe extern "system" fn(
    program: GLuint,
    uniformBlockIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformBlockName: *mut GLchar,
  );
  pub type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(
    program: GLuint,
    uniformBlockIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
  );
  pub type glGetActiveUniformName_t = unsafe extern "system" fn(
    program: GLuint,
    uniformIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformName: *mut GLchar,
  );
  pub type glGetActiveUniformsiv_t = unsafe extern "system" fn(
    program: GLuint,
    uniformCount: GLsizei,
    uniformIndices: *const GLuint,
    pname: GLenum,
    params: *mut GLint,
  );
  pub type glGetAttachedShaders_t = unsafe extern "system" fn(
    program: GLuint,
    maxCount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
  );
  pub type glGetAttribLocation_t =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
  pub type glGetBooleani_v_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean);
  pub type glGetBooleanv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLboolean);
  pub type glGetBufferParameteri64v_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint64);
  pub type glGetBufferParameteriv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
  pub type glGetBufferPointerv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut c_void);
  pub type glGetBufferSubData_t = unsafe extern "system" fn(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut c_void,
  );
  pub type glGetCompressedTexImage_t =
    unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut c_void);
  pub type glGetCompressedTextureImage_t =
    unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void);
  pub type glGetCompressedTextureSubImage_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    bufSize: GLsizei,
    pixels: *mut c_void,
  );
  pub type glGetDebugMessageLog_t = unsafe extern "system" fn(
    count: GLuint,
    bufSize: GLsizei,
    sources: *mut GLenum,
    types: *mut GLenum,
    ids: *mut GLuint,
    severities: *mut GLenum,
    lengths: *mut GLsizei,
    messageLog: *mut GLchar,
  ) -> GLuint;
  pub type glGetDebugMessageLogKHR_t = unsafe extern "system" fn(
    count: GLuint,
    bufSize: GLsizei,
    sources: *mut GLenum,
    types: *mut GLenum,
    ids: *mut GLuint,
    severities: *mut GLenum,
    lengths: *mut GLsizei,
    messageLog: *mut GLchar,
  ) -> GLuint;
  pub type glGetDoublei_v_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLdouble);
  pub type glGetDoublev_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLdouble);
  pub type glGetError_t = unsafe extern "system" fn() -> GLenum;
  pub type glGetFloati_v_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat);
  pub type glGetFloatv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLfloat);
  pub type glGetFragDataIndex_t =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
  pub type glGetFragDataLocation_t =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
  pub type glGetFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
  );
  pub type glGetFramebufferParameteriv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
  pub type glGetGraphicsResetStatus_t = unsafe extern "system" fn() -> GLenum;
  pub type glGetInteger64i_v_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint64);
  pub type glGetInteger64v_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint64);
  pub type glGetIntegeri_v_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint);
  pub type glGetIntegerv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint);
  pub type glGetInternalformati64v_t = unsafe extern "system" fn(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    count: GLsizei,
    params: *mut GLint64,
  );
  pub type glGetInternalformativ_t = unsafe extern "system" fn(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    count: GLsizei,
    params: *mut GLint,
  );
  pub type glGetMultisamplefv_t =
    unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut GLfloat);
  pub type glGetNamedBufferParameteri64v_t =
    unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint64);
  pub type glGetNamedBufferParameteriv_t =
    unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetNamedBufferPointerv_t =
    unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
  pub type glGetNamedBufferSubData_t = unsafe extern "system" fn(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut c_void,
  );
  pub type glGetNamedFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
  );
  pub type glGetNamedFramebufferParameteriv_t =
    unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: *mut GLint);
  pub type glGetNamedRenderbufferParameteriv_t =
    unsafe extern "system" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetObjectLabel_t = unsafe extern "system" fn(
    identifier: GLenum,
    name: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
  );
  pub type glGetObjectLabelKHR_t = unsafe extern "system" fn(
    identifier: GLenum,
    name: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
  );
  pub type glGetObjectPtrLabel_t = unsafe extern "system" fn(
    ptr: *const c_void,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
  );
  pub type glGetObjectPtrLabelKHR_t = unsafe extern "system" fn(
    ptr: *const c_void,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
  );
  pub type glGetPointerv_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut c_void);
  pub type glGetPointervKHR_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut c_void);
  pub type glGetProgramBinary_t = unsafe extern "system" fn(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    binaryFormat: *mut GLenum,
    binary: *mut c_void,
  );
  pub type glGetProgramInfoLog_t = unsafe extern "system" fn(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
  );
  pub type glGetProgramInterfaceiv_t = unsafe extern "system" fn(
    program: GLuint,
    programInterface: GLenum,
    pname: GLenum,
    params: *mut GLint,
  );
  pub type glGetProgramPipelineInfoLog_t = unsafe extern "system" fn(
    pipeline: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
  );
  pub type glGetProgramPipelineiv_t =
    unsafe extern "system" fn(pipeline: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetProgramResourceIndex_t = unsafe extern "system" fn(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
  ) -> GLuint;
  pub type glGetProgramResourceLocation_t = unsafe extern "system" fn(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
  ) -> GLint;
  pub type glGetProgramResourceLocationIndex_t = unsafe extern "system" fn(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
  ) -> GLint;
  pub type glGetProgramResourceName_t = unsafe extern "system" fn(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
  );
  pub type glGetProgramResourceiv_t = unsafe extern "system" fn(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    propCount: GLsizei,
    props: *const GLenum,
    count: GLsizei,
    length: *mut GLsizei,
    params: *mut GLint,
  );
  pub type glGetProgramStageiv_t = unsafe extern "system" fn(
    program: GLuint,
    shadertype: GLenum,
    pname: GLenum,
    values: *mut GLint,
  );
  pub type glGetProgramiv_t =
    unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetQueryBufferObjecti64v_t =
    unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
  pub type glGetQueryBufferObjectiv_t =
    unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
  pub type glGetQueryBufferObjectui64v_t =
    unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
  pub type glGetQueryBufferObjectuiv_t =
    unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
  pub type glGetQueryIndexediv_t =
    unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetQueryObjecti64v_t =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint64);
  pub type glGetQueryObjectiv_t =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetQueryObjectui64v_t =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint64);
  pub type glGetQueryObjectuiv_t =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);
  pub type glGetQueryiv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
  pub type glGetRenderbufferParameteriv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
  pub type glGetSamplerParameterIiv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetSamplerParameterIuiv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);
  pub type glGetSamplerParameterfv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
  pub type glGetSamplerParameteriv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetShaderInfoLog_t = unsafe extern "system" fn(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
  );
  pub type glGetShaderPrecisionFormat_t = unsafe extern "system" fn(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
  );
  pub type glGetShaderSource_t = unsafe extern "system" fn(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
  );
  pub type glGetShaderiv_t =
    unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetString_t = unsafe extern "system" fn(name: GLenum) -> *const GLubyte;
  pub type glGetStringi_t =
    unsafe extern "system" fn(name: GLenum, index: GLuint) -> *const GLubyte;
  pub type glGetSubroutineIndex_t =
    unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint;
  pub type glGetSubroutineUniformLocation_t =
    unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint;
  pub type glGetSynciv_t = unsafe extern "system" fn(
    sync: GLsync,
    pname: GLenum,
    count: GLsizei,
    length: *mut GLsizei,
    values: *mut GLint,
  );
  pub type glGetTexImage_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    format: GLenum,
    ty: GLenum,
    pixels: *mut c_void,
  );
  pub type glGetTexLevelParameterfv_t =
    unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
  pub type glGetTexLevelParameteriv_t =
    unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
  pub type glGetTexParameterIiv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
  pub type glGetTexParameterIuiv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint);
  pub type glGetTexParameterfv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);
  pub type glGetTexParameteriv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
  pub type glGetTextureImage_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    pixels: *mut c_void,
  );
  pub type glGetTextureLevelParameterfv_t =
    unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);
  pub type glGetTextureLevelParameteriv_t =
    unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);
  pub type glGetTextureParameterIiv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetTextureParameterIuiv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLuint);
  pub type glGetTextureParameterfv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLfloat);
  pub type glGetTextureParameteriv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetTextureSubImage_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    pixels: *mut c_void,
  );
  pub type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLsizei,
    ty: *mut GLenum,
    name: *mut GLchar,
  );
  pub type glGetTransformFeedbacki64_v_t =
    unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);
  pub type glGetTransformFeedbacki_v_t =
    unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);
  pub type glGetTransformFeedbackiv_t =
    unsafe extern "system" fn(xfb: GLuint, pname: GLenum, param: *mut GLint);
  pub type glGetUniformBlockIndex_t =
    unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
  pub type glGetUniformIndices_t = unsafe extern "system" fn(
    program: GLuint,
    uniformCount: GLsizei,
    uniformNames: *const *const GLchar,
    uniformIndices: *mut GLuint,
  );
  pub type glGetUniformLocation_t =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
  pub type glGetUniformSubroutineuiv_t =
    unsafe extern "system" fn(shadertype: GLenum, location: GLint, params: *mut GLuint);
  pub type glGetUniformdv_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble);
  pub type glGetUniformfv_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);
  pub type glGetUniformiv_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);
  pub type glGetUniformuiv_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);
  pub type glGetVertexArrayIndexed64iv_t =
    unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);
  pub type glGetVertexArrayIndexediv_t =
    unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
  pub type glGetVertexArrayiv_t =
    unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint);
  pub type glGetVertexAttribIiv_t =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetVertexAttribIuiv_t =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint);
  pub type glGetVertexAttribLdv_t =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
  pub type glGetVertexAttribPointerv_t =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
  pub type glGetVertexAttribdv_t =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
  pub type glGetVertexAttribfv_t =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);
  pub type glGetVertexAttribiv_t =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
  pub type glGetnColorTable_t = unsafe extern "system" fn(
    target: GLenum,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    table: *mut c_void,
  );
  pub type glGetnCompressedTexImage_t =
    unsafe extern "system" fn(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void);
  pub type glGetnConvolutionFilter_t = unsafe extern "system" fn(
    target: GLenum,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    image: *mut c_void,
  );
  pub type glGetnHistogram_t = unsafe extern "system" fn(
    target: GLenum,
    reset: GLboolean,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    values: *mut c_void,
  );
  pub type glGetnMapdv_t =
    unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble);
  pub type glGetnMapfv_t =
    unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat);
  pub type glGetnMapiv_t =
    unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint);
  pub type glGetnMinmax_t = unsafe extern "system" fn(
    target: GLenum,
    reset: GLboolean,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    values: *mut c_void,
  );
  pub type glGetnPixelMapfv_t =
    unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLfloat);
  pub type glGetnPixelMapuiv_t =
    unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLuint);
  pub type glGetnPixelMapusv_t =
    unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLushort);
  pub type glGetnPolygonStipple_t =
    unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);
  pub type glGetnSeparableFilter_t = unsafe extern "system" fn(
    target: GLenum,
    format: GLenum,
    ty: GLenum,
    rowBufSize: GLsizei,
    row: *mut c_void,
    columnBufSize: GLsizei,
    column: *mut c_void,
    span: *mut c_void,
  );
  pub type glGetnTexImage_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    pixels: *mut c_void,
  );
  pub type glGetnUniformdv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLdouble,
  );
  pub type glGetnUniformfv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLfloat,
  );
  pub type glGetnUniformiv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLint,
  );
  pub type glGetnUniformuiv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLuint,
  );
  pub type glHint_t = unsafe extern "system" fn(target: GLenum, mode: GLenum);
  pub type glInvalidateBufferData_t = unsafe extern "system" fn(buffer: GLuint);
  pub type glInvalidateBufferSubData_t =
    unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
  pub type glInvalidateFramebuffer_t =
    unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
  pub type glInvalidateNamedFramebufferData_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: *const GLenum,
  );
  pub type glInvalidateNamedFramebufferSubData_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glInvalidateSubFramebuffer_t = unsafe extern "system" fn(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glInvalidateTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint);
  pub type glInvalidateTexSubImage_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
  );
  pub type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;
  pub type glIsEnabled_t = unsafe extern "system" fn(cap: GLenum) -> GLboolean;
  pub type glIsEnabledi_t = unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;
  pub type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;
  pub type glIsProgram_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;
  pub type glIsProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;
  pub type glIsQuery_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;
  pub type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;
  pub type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;
  pub type glIsShader_t = unsafe extern "system" fn(shader: GLuint) -> GLboolean;
  pub type glIsSync_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;
  pub type glIsTexture_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;
  pub type glIsTransformFeedback_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;
  pub type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;
  pub type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);
  pub type glLinkProgram_t = unsafe extern "system" fn(program: GLuint);
  pub type glLogicOp_t = unsafe extern "system" fn(opcode: GLenum);
  pub type glMapBuffer_t = unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut c_void;
  pub type glMapBufferRange_t = unsafe extern "system" fn(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
  ) -> *mut c_void;
  pub type glMapNamedBuffer_t =
    unsafe extern "system" fn(buffer: GLuint, access: GLenum) -> *mut c_void;
  pub type glMapNamedBufferRange_t = unsafe extern "system" fn(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
  ) -> *mut c_void;
  pub type glMemoryBarrier_t = unsafe extern "system" fn(barriers: GLbitfield);
  pub type glMemoryBarrierByRegion_t = unsafe extern "system" fn(barriers: GLbitfield);
  pub type glMinSampleShading_t = unsafe extern "system" fn(value: GLfloat);
  pub type glMultiDrawArrays_t = unsafe extern "system" fn(
    mode: GLenum,
    first: *const GLint,
    count: *const GLsizei,
    drawcount: GLsizei,
  );
  pub type glMultiDrawArraysIndirect_t = unsafe extern "system" fn(
    mode: GLenum,
    indirect: *const c_void,
    drawcount: GLsizei,
    stride: GLsizei,
  );
  pub type glMultiDrawArraysIndirectCount_t = unsafe extern "system" fn(
    mode: GLenum,
    indirect: *const c_void,
    drawcount: GLintptr,
    maxdrawcount: GLsizei,
    stride: GLsizei,
  );
  pub type glMultiDrawElements_t = unsafe extern "system" fn(
    mode: GLenum,
    count: *const GLsizei,
    ty: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
  );
  pub type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(
    mode: GLenum,
    count: *const GLsizei,
    ty: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
    basevertex: *const GLint,
  );
  pub type glMultiDrawElementsIndirect_t = unsafe extern "system" fn(
    mode: GLenum,
    ty: GLenum,
    indirect: *const c_void,
    drawcount: GLsizei,
    stride: GLsizei,
  );
  pub type glMultiDrawElementsIndirectCount_t = unsafe extern "system" fn(
    mode: GLenum,
    ty: GLenum,
    indirect: *const c_void,
    drawcount: GLintptr,
    maxdrawcount: GLsizei,
    stride: GLsizei,
  );
  pub type glMultiTexCoordP1ui_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
  pub type glMultiTexCoordP1uiv_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
  pub type glMultiTexCoordP2ui_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
  pub type glMultiTexCoordP2uiv_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
  pub type glMultiTexCoordP3ui_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
  pub type glMultiTexCoordP3uiv_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
  pub type glMultiTexCoordP4ui_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
  pub type glMultiTexCoordP4uiv_t =
    unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
  pub type glNamedBufferData_t =
    unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
  pub type glNamedBufferStorage_t = unsafe extern "system" fn(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const c_void,
    flags: GLbitfield,
  );
  pub type glNamedBufferSubData_t = unsafe extern "system" fn(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
  );
  pub type glNamedFramebufferDrawBuffer_t =
    unsafe extern "system" fn(framebuffer: GLuint, buf: GLenum);
  pub type glNamedFramebufferDrawBuffers_t =
    unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
  pub type glNamedFramebufferParameteri_t =
    unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: GLint);
  pub type glNamedFramebufferReadBuffer_t =
    unsafe extern "system" fn(framebuffer: GLuint, src: GLenum);
  pub type glNamedFramebufferRenderbuffer_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
  );
  pub type glNamedFramebufferTexture_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
  );
  pub type glNamedFramebufferTextureLayer_t = unsafe extern "system" fn(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
  );
  pub type glNamedRenderbufferStorage_t = unsafe extern "system" fn(
    renderbuffer: GLuint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glNamedRenderbufferStorageMultisample_t = unsafe extern "system" fn(
    renderbuffer: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glNormalP3ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
  pub type glNormalP3uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
  pub type glObjectLabel_t = unsafe extern "system" fn(
    identifier: GLenum,
    name: GLuint,
    length: GLsizei,
    label: *const GLchar,
  );
  pub type glObjectLabelKHR_t = unsafe extern "system" fn(
    identifier: GLenum,
    name: GLuint,
    length: GLsizei,
    label: *const GLchar,
  );
  pub type glObjectPtrLabel_t =
    unsafe extern "system" fn(ptr: *const c_void, length: GLsizei, label: *const GLchar);
  pub type glObjectPtrLabelKHR_t =
    unsafe extern "system" fn(ptr: *const c_void, length: GLsizei, label: *const GLchar);
  pub type glPatchParameterfv_t = unsafe extern "system" fn(pname: GLenum, values: *const GLfloat);
  pub type glPatchParameteri_t = unsafe extern "system" fn(pname: GLenum, value: GLint);
  pub type glPauseTransformFeedback_t = unsafe extern "system" fn();
  pub type glPixelStoref_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
  pub type glPixelStorei_t = unsafe extern "system" fn(pname: GLenum, param: GLint);
  pub type glPointParameterf_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
  pub type glPointParameterfv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);
  pub type glPointParameteri_t = unsafe extern "system" fn(pname: GLenum, param: GLint);
  pub type glPointParameteriv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);
  pub type glPointSize_t = unsafe extern "system" fn(size: GLfloat);
  pub type glPolygonMode_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);
  pub type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);
  pub type glPolygonOffsetClamp_t =
    unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);
  pub type glPopDebugGroup_t = unsafe extern "system" fn();
  pub type glPopDebugGroupKHR_t = unsafe extern "system" fn();
  pub type glPrimitiveBoundingBox_t = unsafe extern "system" fn(
    minX: GLfloat,
    minY: GLfloat,
    minZ: GLfloat,
    minW: GLfloat,
    maxX: GLfloat,
    maxY: GLfloat,
    maxZ: GLfloat,
    maxW: GLfloat,
  );
  pub type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);
  pub type glProgramBinary_t = unsafe extern "system" fn(
    program: GLuint,
    binaryFormat: GLenum,
    binary: *const c_void,
    length: GLsizei,
  );
  pub type glProgramParameteri_t =
    unsafe extern "system" fn(program: GLuint, pname: GLenum, value: GLint);
  pub type glProgramUniform1d_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble);
  pub type glProgramUniform1dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
  );
  pub type glProgramUniform1f_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);
  pub type glProgramUniform1fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  );
  pub type glProgramUniform1i_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);
  pub type glProgramUniform1iv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  );
  pub type glProgramUniform1ui_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);
  pub type glProgramUniform1uiv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  );
  pub type glProgramUniform2d_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
  pub type glProgramUniform2dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
  );
  pub type glProgramUniform2f_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
  pub type glProgramUniform2fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  );
  pub type glProgramUniform2i_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);
  pub type glProgramUniform2iv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  );
  pub type glProgramUniform2ui_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
  pub type glProgramUniform2uiv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  );
  pub type glProgramUniform3d_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
  );
  pub type glProgramUniform3dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
  );
  pub type glProgramUniform3f_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
  );
  pub type glProgramUniform3fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  );
  pub type glProgramUniform3i_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
  pub type glProgramUniform3iv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  );
  pub type glProgramUniform3ui_t =
    unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
  pub type glProgramUniform3uiv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  );
  pub type glProgramUniform4d_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
    v3: GLdouble,
  );
  pub type glProgramUniform4dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
  );
  pub type glProgramUniform4f_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
  );
  pub type glProgramUniform4fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  );
  pub type glProgramUniform4i_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
    v3: GLint,
  );
  pub type glProgramUniform4iv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  );
  pub type glProgramUniform4ui_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
    v3: GLuint,
  );
  pub type glProgramUniform4uiv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  );
  pub type glProgramUniformMatrix2dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix2fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix2x3dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix2x3fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix2x4dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix2x4fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix3dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix3fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix3x2dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix3x2fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix3x4dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix3x4fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix4dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix4fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix4x2dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix4x2fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProgramUniformMatrix4x3dv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glProgramUniformMatrix4x3fv_t = unsafe extern "system" fn(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glProvokingVertex_t = unsafe extern "system" fn(mode: GLenum);
  pub type glPushDebugGroup_t =
    unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
  pub type glPushDebugGroupKHR_t =
    unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
  pub type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: GLenum);
  pub type glReadBuffer_t = unsafe extern "system" fn(src: GLenum);
  pub type glReadPixels_t = unsafe extern "system" fn(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    ty: GLenum,
    pixels: *mut c_void,
  );
  pub type glReadnPixels_t = unsafe extern "system" fn(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    ty: GLenum,
    bufSize: GLsizei,
    data: *mut c_void,
  );
  pub type glReleaseShaderCompiler_t = unsafe extern "system" fn();
  pub type glRenderbufferStorage_t = unsafe extern "system" fn(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glResumeTransformFeedback_t = unsafe extern "system" fn();
  pub type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);
  pub type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);
  pub type glSamplerParameterIiv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
  pub type glSamplerParameterIuiv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);
  pub type glSamplerParameterf_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLfloat);
  pub type glSamplerParameterfv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat);
  pub type glSamplerParameteri_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLint);
  pub type glSamplerParameteriv_t =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
  pub type glScissor_t =
    unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  pub type glScissorArrayv_t =
    unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);
  pub type glScissorIndexed_t = unsafe extern "system" fn(
    index: GLuint,
    left: GLint,
    bottom: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glScissorIndexedv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
  pub type glSecondaryColorP3ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
  pub type glSecondaryColorP3uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
  pub type glShaderBinary_t = unsafe extern "system" fn(
    count: GLsizei,
    shaders: *const GLuint,
    binaryFormat: GLenum,
    binary: *const c_void,
    length: GLsizei,
  );
  pub type glShaderSource_t = unsafe extern "system" fn(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
  );
  pub type glShaderStorageBlockBinding_t = unsafe extern "system" fn(
    program: GLuint,
    storageBlockIndex: GLuint,
    storageBlockBinding: GLuint,
  );
  pub type glSpecializeShader_t = unsafe extern "system" fn(
    shader: GLuint,
    pEntryPoint: *const GLchar,
    numSpecializationConstants: GLuint,
    pConstantIndex: *const GLuint,
    pConstantValue: *const GLuint,
  );
  pub type glStencilFunc_t =
    unsafe extern "system" fn(func: GLenum, reference: GLint, mask: GLuint);
  pub type glStencilFuncSeparate_t =
    unsafe extern "system" fn(face: GLenum, func: GLenum, reference: GLint, mask: GLuint);
  pub type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);
  pub type glStencilMaskSeparate_t = unsafe extern "system" fn(face: GLenum, mask: GLuint);
  pub type glStencilOp_t = unsafe extern "system" fn(fail: GLenum, zfail: GLenum, zpass: GLenum);
  pub type glStencilOpSeparate_t =
    unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
  pub type glTexBuffer_t =
    unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);
  pub type glTexBufferRange_t = unsafe extern "system" fn(
    target: GLenum,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
  );
  pub type glTexCoordP1ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
  pub type glTexCoordP1uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
  pub type glTexCoordP2ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
  pub type glTexCoordP2uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
  pub type glTexCoordP3ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
  pub type glTexCoordP3uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
  pub type glTexCoordP4ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
  pub type glTexCoordP4uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
  pub type glTexImage1D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    border: GLint,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTexImage2D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTexImage2DMultisample_t = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  pub type glTexImage3D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTexImage3DMultisample_t = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  pub type glTexParameterIiv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
  pub type glTexParameterIuiv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint);
  pub type glTexParameterf_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);
  pub type glTexParameterfv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);
  pub type glTexParameteri_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
  pub type glTexParameteriv_t =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
  pub type glTexStorage1D_t = unsafe extern "system" fn(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
  );
  pub type glTexStorage2D_t = unsafe extern "system" fn(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glTexStorage2DMultisample_t = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  pub type glTexStorage3D_t = unsafe extern "system" fn(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
  );
  pub type glTexStorage3DMultisample_t = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  pub type glTexSubImage1D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTexSubImage2D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTexSubImage3D_t = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTextureBarrier_t = unsafe extern "system" fn();
  pub type glTextureBuffer_t =
    unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint);
  pub type glTextureBufferRange_t = unsafe extern "system" fn(
    texture: GLuint,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
  );
  pub type glTextureParameterIiv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLint);
  pub type glTextureParameterIuiv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLuint);
  pub type glTextureParameterf_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLfloat);
  pub type glTextureParameterfv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLfloat);
  pub type glTextureParameteri_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLint);
  pub type glTextureParameteriv_t =
    unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLint);
  pub type glTextureStorage1D_t = unsafe extern "system" fn(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
  );
  pub type glTextureStorage2D_t = unsafe extern "system" fn(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  pub type glTextureStorage2DMultisample_t = unsafe extern "system" fn(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  pub type glTextureStorage3D_t = unsafe extern "system" fn(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
  );
  pub type glTextureStorage3DMultisample_t = unsafe extern "system" fn(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  pub type glTextureSubImage1D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTextureSubImage2D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTextureSubImage3D_t = unsafe extern "system" fn(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    ty: GLenum,
    pixels: *const c_void,
  );
  pub type glTextureView_t = unsafe extern "system" fn(
    texture: GLuint,
    target: GLenum,
    origtexture: GLuint,
    internalformat: GLenum,
    minlevel: GLuint,
    numlevels: GLuint,
    minlayer: GLuint,
    numlayers: GLuint,
  );
  pub type glTransformFeedbackBufferBase_t =
    unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint);
  pub type glTransformFeedbackBufferRange_t = unsafe extern "system" fn(
    xfb: GLuint,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
  );
  pub type glTransformFeedbackVaryings_t = unsafe extern "system" fn(
    program: GLuint,
    count: GLsizei,
    varyings: *const *const GLchar,
    bufferMode: GLenum,
  );
  pub type glUniform1d_t = unsafe extern "system" fn(location: GLint, x: GLdouble);
  pub type glUniform1dv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
  pub type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);
  pub type glUniform1fv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
  pub type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);
  pub type glUniform1iv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
  pub type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);
  pub type glUniform1uiv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
  pub type glUniform2d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble);
  pub type glUniform2dv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
  pub type glUniform2f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);
  pub type glUniform2fv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
  pub type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);
  pub type glUniform2iv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
  pub type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);
  pub type glUniform2uiv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
  pub type glUniform3d_t =
    unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
  pub type glUniform3dv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
  pub type glUniform3f_t =
    unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
  pub type glUniform3fv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
  pub type glUniform3i_t =
    unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);
  pub type glUniform3iv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
  pub type glUniform3ui_t =
    unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
  pub type glUniform3uiv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
  pub type glUniform4d_t =
    unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
  pub type glUniform4dv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
  pub type glUniform4f_t =
    unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
  pub type glUniform4fv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
  pub type glUniform4i_t =
    unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
  pub type glUniform4iv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
  pub type glUniform4ui_t =
    unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
  pub type glUniform4uiv_t =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
  pub type glUniformBlockBinding_t = unsafe extern "system" fn(
    program: GLuint,
    uniformBlockIndex: GLuint,
    uniformBlockBinding: GLuint,
  );
  pub type glUniformMatrix2dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix2fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix2x3dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix2x3fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix2x4dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix2x4fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix3dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix3fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix3x2dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix3x2fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix3x4dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix3x4fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix4dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix4fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix4x2dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix4x2fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformMatrix4x3dv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
  );
  pub type glUniformMatrix4x3fv_t = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  pub type glUniformSubroutinesuiv_t =
    unsafe extern "system" fn(shadertype: GLenum, count: GLsizei, indices: *const GLuint);
  pub type glUnmapBuffer_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;
  pub type glUnmapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;
  pub type glUseProgram_t = unsafe extern "system" fn(program: GLuint);
  pub type glUseProgramStages_t =
    unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);
  pub type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);
  pub type glValidateProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);
  pub type glVertexArrayAttribBinding_t =
    unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
  pub type glVertexArrayAttribFormat_t = unsafe extern "system" fn(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    ty: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
  );
  pub type glVertexArrayAttribIFormat_t = unsafe extern "system" fn(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    ty: GLenum,
    relativeoffset: GLuint,
  );
  pub type glVertexArrayAttribLFormat_t = unsafe extern "system" fn(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    ty: GLenum,
    relativeoffset: GLuint,
  );
  pub type glVertexArrayBindingDivisor_t =
    unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
  pub type glVertexArrayElementBuffer_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint);
  pub type glVertexArrayVertexBuffer_t = unsafe extern "system" fn(
    vaobj: GLuint,
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
  );
  pub type glVertexArrayVertexBuffers_t = unsafe extern "system" fn(
    vaobj: GLuint,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
  );
  pub type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);
  pub type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);
  pub type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
  pub type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);
  pub type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
  pub type glVertexAttrib2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);
  pub type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttrib2f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);
  pub type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
  pub type glVertexAttrib2s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);
  pub type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
  pub type glVertexAttrib3d_t =
    unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
  pub type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttrib3f_t =
    unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
  pub type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
  pub type glVertexAttrib3s_t =
    unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
  pub type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
  pub type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
  pub type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
  pub type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
  pub type glVertexAttrib4Nub_t =
    unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
  pub type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
  pub type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
  pub type glVertexAttrib4Nusv_t = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
  pub type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
  pub type glVertexAttrib4d_t =
    unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
  pub type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttrib4f_t =
    unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
  pub type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
  pub type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
  pub type glVertexAttrib4s_t =
    unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
  pub type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
  pub type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
  pub type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
  pub type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
  pub type glVertexAttribBinding_t =
    unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint);
  pub type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);
  pub type glVertexAttribFormat_t = unsafe extern "system" fn(
    attribindex: GLuint,
    size: GLint,
    ty: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
  );
  pub type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);
  pub type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
  pub type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);
  pub type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
  pub type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);
  pub type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
  pub type glVertexAttribI2ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);
  pub type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
  pub type glVertexAttribI3i_t =
    unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);
  pub type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
  pub type glVertexAttribI3ui_t =
    unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);
  pub type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
  pub type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
  pub type glVertexAttribI4i_t =
    unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
  pub type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
  pub type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
  pub type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
  pub type glVertexAttribI4ui_t =
    unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
  pub type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
  pub type glVertexAttribI4usv_t = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
  pub type glVertexAttribIFormat_t =
    unsafe extern "system" fn(attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint);
  pub type glVertexAttribIPointer_t = unsafe extern "system" fn(
    index: GLuint,
    size: GLint,
    ty: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
  );
  pub type glVertexAttribL1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);
  pub type glVertexAttribL1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttribL2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);
  pub type glVertexAttribL2dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttribL3d_t =
    unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
  pub type glVertexAttribL3dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttribL4d_t =
    unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
  pub type glVertexAttribL4dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
  pub type glVertexAttribLFormat_t =
    unsafe extern "system" fn(attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint);
  pub type glVertexAttribLPointer_t = unsafe extern "system" fn(
    index: GLuint,
    size: GLint,
    ty: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
  );
  pub type glVertexAttribP1ui_t =
    unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
  pub type glVertexAttribP1uiv_t = unsafe extern "system" fn(
    index: GLuint,
    ty: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  pub type glVertexAttribP2ui_t =
    unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
  pub type glVertexAttribP2uiv_t = unsafe extern "system" fn(
    index: GLuint,
    ty: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  pub type glVertexAttribP3ui_t =
    unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
  pub type glVertexAttribP3uiv_t = unsafe extern "system" fn(
    index: GLuint,
    ty: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  pub type glVertexAttribP4ui_t =
    unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
  pub type glVertexAttribP4uiv_t = unsafe extern "system" fn(
    index: GLuint,
    ty: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  pub type glVertexAttribPointer_t = unsafe extern "system" fn(
    index: GLuint,
    size: GLint,
    ty: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
  );
  pub type glVertexBindingDivisor_t =
    unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint);
  pub type glVertexP2ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
  pub type glVertexP2uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
  pub type glVertexP3ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
  pub type glVertexP3uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
  pub type glVertexP4ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
  pub type glVertexP4uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
  pub type glViewport_t =
    unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  pub type glViewportArrayv_t =
    unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);
  pub type glViewportIndexedf_t =
    unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
  pub type glViewportIndexedfv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
  pub type glWaitSync_t =
    unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
}
impl GlFns {
  pub fn has_loaded(&self) -> FnLoadedChecker<'_> {
    FnLoadedChecker(self)
  }
}

macro_rules! mk_load_checker_method {
  ($full_name:ident, $short_name:ident) => {
    #[inline]
    #[must_use]
    pub fn $short_name(&self) -> bool {
      {
        self.0.$full_name.is_some()
      }
    }
  };
}

pub struct FnLoadedChecker<'g>(&'g GlFns);
impl FnLoadedChecker<'_> {
  mk_load_checker_method!(glActiveShaderProgram, ActiveShaderProgram);
  mk_load_checker_method!(glActiveTexture, ActiveTexture);
  mk_load_checker_method!(glAttachShader, AttachShader);
  mk_load_checker_method!(glBeginConditionalRender, BeginConditionalRender);
  mk_load_checker_method!(glBeginQuery, BeginQuery);
  mk_load_checker_method!(glBeginQueryIndexed, BeginQueryIndexed);
  mk_load_checker_method!(glBeginTransformFeedback, BeginTransformFeedback);
  mk_load_checker_method!(glBindAttribLocation, BindAttribLocation);
  mk_load_checker_method!(glBindBuffer, BindBuffer);
  mk_load_checker_method!(glBindBufferBase, BindBufferBase);
  mk_load_checker_method!(glBindBufferRange, BindBufferRange);
  mk_load_checker_method!(glBindBuffersBase, BindBuffersBase);
  mk_load_checker_method!(glBindBuffersRange, BindBuffersRange);
  mk_load_checker_method!(glBindFragDataLocation, BindFragDataLocation);
  mk_load_checker_method!(glBindFragDataLocationIndexed, BindFragDataLocationIndexed);
  mk_load_checker_method!(glBindFramebuffer, BindFramebuffer);
  mk_load_checker_method!(glBindImageTexture, BindImageTexture);
  mk_load_checker_method!(glBindImageTextures, BindImageTextures);
  mk_load_checker_method!(glBindProgramPipeline, BindProgramPipeline);
  mk_load_checker_method!(glBindRenderbuffer, BindRenderbuffer);
  mk_load_checker_method!(glBindSampler, BindSampler);
  mk_load_checker_method!(glBindSamplers, BindSamplers);
  mk_load_checker_method!(glBindTexture, BindTexture);
  mk_load_checker_method!(glBindTextureUnit, BindTextureUnit);
  mk_load_checker_method!(glBindTextures, BindTextures);
  mk_load_checker_method!(glBindTransformFeedback, BindTransformFeedback);
  mk_load_checker_method!(glBindVertexArray, BindVertexArray);
  mk_load_checker_method!(glBindVertexBuffer, BindVertexBuffer);
  mk_load_checker_method!(glBindVertexBuffers, BindVertexBuffers);
  mk_load_checker_method!(glBlendBarrier, BlendBarrier);
  mk_load_checker_method!(glBlendColor, BlendColor);
  mk_load_checker_method!(glBlendEquation, BlendEquation);
  mk_load_checker_method!(glBlendEquationSeparate, BlendEquationSeparate);
  mk_load_checker_method!(glBlendEquationSeparatei, BlendEquationSeparatei);
  mk_load_checker_method!(glBlendEquationi, BlendEquationi);
  mk_load_checker_method!(glBlendFunc, BlendFunc);
  mk_load_checker_method!(glBlendFuncSeparate, BlendFuncSeparate);
  mk_load_checker_method!(glBlendFuncSeparatei, BlendFuncSeparatei);
  mk_load_checker_method!(glBlendFunci, BlendFunci);
  mk_load_checker_method!(glBlitFramebuffer, BlitFramebuffer);
  mk_load_checker_method!(glBlitNamedFramebuffer, BlitNamedFramebuffer);
  mk_load_checker_method!(glBufferData, BufferData);
  mk_load_checker_method!(glBufferStorage, BufferStorage);
  mk_load_checker_method!(glBufferSubData, BufferSubData);
  mk_load_checker_method!(glCheckFramebufferStatus, CheckFramebufferStatus);
  mk_load_checker_method!(glCheckNamedFramebufferStatus, CheckNamedFramebufferStatus);
  mk_load_checker_method!(glClampColor, ClampColor);
  mk_load_checker_method!(glClear, Clear);
  mk_load_checker_method!(glClearBufferData, ClearBufferData);
  mk_load_checker_method!(glClearBufferSubData, ClearBufferSubData);
  mk_load_checker_method!(glClearBufferfi, ClearBufferfi);
  mk_load_checker_method!(glClearBufferfv, ClearBufferfv);
  mk_load_checker_method!(glClearBufferiv, ClearBufferiv);
  mk_load_checker_method!(glClearBufferuiv, ClearBufferuiv);
  mk_load_checker_method!(glClearColor, ClearColor);
  mk_load_checker_method!(glClearDepth, ClearDepth);
  mk_load_checker_method!(glClearDepthf, ClearDepthf);
  mk_load_checker_method!(glClearNamedBufferData, ClearNamedBufferData);
  mk_load_checker_method!(glClearNamedBufferSubData, ClearNamedBufferSubData);
  mk_load_checker_method!(glClearNamedFramebufferfi, ClearNamedFramebufferfi);
  mk_load_checker_method!(glClearNamedFramebufferfv, ClearNamedFramebufferfv);
  mk_load_checker_method!(glClearNamedFramebufferiv, ClearNamedFramebufferiv);
  mk_load_checker_method!(glClearNamedFramebufferuiv, ClearNamedFramebufferuiv);
  mk_load_checker_method!(glClearStencil, ClearStencil);
  mk_load_checker_method!(glClearTexImage, ClearTexImage);
  mk_load_checker_method!(glClearTexSubImage, ClearTexSubImage);
  mk_load_checker_method!(glClientWaitSync, ClientWaitSync);
  mk_load_checker_method!(glClipControl, ClipControl);
  mk_load_checker_method!(glColorMask, ColorMask);
  mk_load_checker_method!(glColorMaski, ColorMaski);
  mk_load_checker_method!(glColorP3ui, ColorP3ui);
  mk_load_checker_method!(glColorP3uiv, ColorP3uiv);
  mk_load_checker_method!(glColorP4ui, ColorP4ui);
  mk_load_checker_method!(glColorP4uiv, ColorP4uiv);
  mk_load_checker_method!(glCompileShader, CompileShader);
  mk_load_checker_method!(glCompressedTexImage1D, CompressedTexImage1D);
  mk_load_checker_method!(glCompressedTexImage2D, CompressedTexImage2D);
  mk_load_checker_method!(glCompressedTexImage3D, CompressedTexImage3D);
  mk_load_checker_method!(glCompressedTexSubImage1D, CompressedTexSubImage1D);
  mk_load_checker_method!(glCompressedTexSubImage2D, CompressedTexSubImage2D);
  mk_load_checker_method!(glCompressedTexSubImage3D, CompressedTexSubImage3D);
  mk_load_checker_method!(glCompressedTextureSubImage1D, CompressedTextureSubImage1D);
  mk_load_checker_method!(glCompressedTextureSubImage2D, CompressedTextureSubImage2D);
  mk_load_checker_method!(glCompressedTextureSubImage3D, CompressedTextureSubImage3D);
  mk_load_checker_method!(glCopyBufferSubData, CopyBufferSubData);
  mk_load_checker_method!(glCopyImageSubData, CopyImageSubData);
  mk_load_checker_method!(glCopyNamedBufferSubData, CopyNamedBufferSubData);
  mk_load_checker_method!(glCopyTexImage1D, CopyTexImage1D);
  mk_load_checker_method!(glCopyTexImage2D, CopyTexImage2D);
  mk_load_checker_method!(glCopyTexSubImage1D, CopyTexSubImage1D);
  mk_load_checker_method!(glCopyTexSubImage2D, CopyTexSubImage2D);
  mk_load_checker_method!(glCopyTexSubImage3D, CopyTexSubImage3D);
  mk_load_checker_method!(glCopyTextureSubImage1D, CopyTextureSubImage1D);
  mk_load_checker_method!(glCopyTextureSubImage2D, CopyTextureSubImage2D);
  mk_load_checker_method!(glCopyTextureSubImage3D, CopyTextureSubImage3D);
  mk_load_checker_method!(glCreateBuffers, CreateBuffers);
  mk_load_checker_method!(glCreateFramebuffers, CreateFramebuffers);
  mk_load_checker_method!(glCreateProgram, CreateProgram);
  mk_load_checker_method!(glCreateProgramPipelines, CreateProgramPipelines);
  mk_load_checker_method!(glCreateQueries, CreateQueries);
  mk_load_checker_method!(glCreateRenderbuffers, CreateRenderbuffers);
  mk_load_checker_method!(glCreateSamplers, CreateSamplers);
  mk_load_checker_method!(glCreateShader, CreateShader);
  mk_load_checker_method!(glCreateShaderProgramv, CreateShaderProgramv);
  mk_load_checker_method!(glCreateTextures, CreateTextures);
  mk_load_checker_method!(glCreateTransformFeedbacks, CreateTransformFeedbacks);
  mk_load_checker_method!(glCreateVertexArrays, CreateVertexArrays);
  mk_load_checker_method!(glCullFace, CullFace);
  mk_load_checker_method!(glDebugMessageCallback, DebugMessageCallback);
  mk_load_checker_method!(glDebugMessageCallbackKHR, DebugMessageCallbackKHR);
  mk_load_checker_method!(glDebugMessageControl, DebugMessageControl);
  mk_load_checker_method!(glDebugMessageControlKHR, DebugMessageControlKHR);
  mk_load_checker_method!(glDebugMessageInsert, DebugMessageInsert);
  mk_load_checker_method!(glDebugMessageInsertKHR, DebugMessageInsertKHR);
  mk_load_checker_method!(glDeleteBuffers, DeleteBuffers);
  mk_load_checker_method!(glDeleteFramebuffers, DeleteFramebuffers);
  mk_load_checker_method!(glDeleteProgram, DeleteProgram);
  mk_load_checker_method!(glDeleteProgramPipelines, DeleteProgramPipelines);
  mk_load_checker_method!(glDeleteQueries, DeleteQueries);
  mk_load_checker_method!(glDeleteRenderbuffers, DeleteRenderbuffers);
  mk_load_checker_method!(glDeleteSamplers, DeleteSamplers);
  mk_load_checker_method!(glDeleteShader, DeleteShader);
  mk_load_checker_method!(glDeleteSync, DeleteSync);
  mk_load_checker_method!(glDeleteTextures, DeleteTextures);
  mk_load_checker_method!(glDeleteTransformFeedbacks, DeleteTransformFeedbacks);
  mk_load_checker_method!(glDeleteVertexArrays, DeleteVertexArrays);
  mk_load_checker_method!(glDepthFunc, DepthFunc);
  mk_load_checker_method!(glDepthMask, DepthMask);
  mk_load_checker_method!(glDepthRange, DepthRange);
  mk_load_checker_method!(glDepthRangeArrayv, DepthRangeArrayv);
  mk_load_checker_method!(glDepthRangeIndexed, DepthRangeIndexed);
  mk_load_checker_method!(glDepthRangef, DepthRangef);
  mk_load_checker_method!(glDetachShader, DetachShader);
  mk_load_checker_method!(glDisable, Disable);
  mk_load_checker_method!(glDisableVertexArrayAttrib, DisableVertexArrayAttrib);
  mk_load_checker_method!(glDisableVertexAttribArray, DisableVertexAttribArray);
  mk_load_checker_method!(glDisablei, Disablei);
  mk_load_checker_method!(glDispatchCompute, DispatchCompute);
  mk_load_checker_method!(glDispatchComputeIndirect, DispatchComputeIndirect);
  mk_load_checker_method!(glDrawArrays, DrawArrays);
  mk_load_checker_method!(glDrawArraysIndirect, DrawArraysIndirect);
  mk_load_checker_method!(glDrawArraysInstanced, DrawArraysInstanced);
  mk_load_checker_method!(glDrawArraysInstancedBaseInstance, DrawArraysInstancedBaseInstance);
  mk_load_checker_method!(glDrawBuffer, DrawBuffer);
  mk_load_checker_method!(glDrawBuffers, DrawBuffers);
  mk_load_checker_method!(glDrawElements, DrawElements);
  mk_load_checker_method!(glDrawElementsBaseVertex, DrawElementsBaseVertex);
  mk_load_checker_method!(glDrawElementsIndirect, DrawElementsIndirect);
  mk_load_checker_method!(glDrawElementsInstanced, DrawElementsInstanced);
  mk_load_checker_method!(glDrawElementsInstancedBaseInstance, DrawElementsInstancedBaseInstance);
  mk_load_checker_method!(glDrawElementsInstancedBaseVertex, DrawElementsInstancedBaseVertex);
  mk_load_checker_method!(
    glDrawElementsInstancedBaseVertexBaseInstance,
    DrawElementsInstancedBaseVertexBaseInstance
  );
  mk_load_checker_method!(glDrawRangeElements, DrawRangeElements);
  mk_load_checker_method!(glDrawRangeElementsBaseVertex, DrawRangeElementsBaseVertex);
  mk_load_checker_method!(glDrawTransformFeedback, DrawTransformFeedback);
  mk_load_checker_method!(glDrawTransformFeedbackInstanced, DrawTransformFeedbackInstanced);
  mk_load_checker_method!(glDrawTransformFeedbackStream, DrawTransformFeedbackStream);
  mk_load_checker_method!(
    glDrawTransformFeedbackStreamInstanced,
    DrawTransformFeedbackStreamInstanced
  );
  mk_load_checker_method!(glEnable, Enable);
  mk_load_checker_method!(glEnableVertexArrayAttrib, EnableVertexArrayAttrib);
  mk_load_checker_method!(glEnableVertexAttribArray, EnableVertexAttribArray);
  mk_load_checker_method!(glEnablei, Enablei);
  mk_load_checker_method!(glEndConditionalRender, EndConditionalRender);
  mk_load_checker_method!(glEndQuery, EndQuery);
  mk_load_checker_method!(glEndQueryIndexed, EndQueryIndexed);
  mk_load_checker_method!(glEndTransformFeedback, EndTransformFeedback);
  mk_load_checker_method!(glFenceSync, FenceSync);
  mk_load_checker_method!(glFinish, Finish);
  mk_load_checker_method!(glFlush, Flush);
  mk_load_checker_method!(glFlushMappedBufferRange, FlushMappedBufferRange);
  mk_load_checker_method!(glFlushMappedNamedBufferRange, FlushMappedNamedBufferRange);
  mk_load_checker_method!(glFramebufferParameteri, FramebufferParameteri);
  mk_load_checker_method!(glFramebufferRenderbuffer, FramebufferRenderbuffer);
  mk_load_checker_method!(glFramebufferTexture, FramebufferTexture);
  mk_load_checker_method!(glFramebufferTexture1D, FramebufferTexture1D);
  mk_load_checker_method!(glFramebufferTexture2D, FramebufferTexture2D);
  mk_load_checker_method!(glFramebufferTexture3D, FramebufferTexture3D);
  mk_load_checker_method!(glFramebufferTextureLayer, FramebufferTextureLayer);
  mk_load_checker_method!(glFrontFace, FrontFace);
  mk_load_checker_method!(glGenBuffers, GenBuffers);
  mk_load_checker_method!(glGenFramebuffers, GenFramebuffers);
  mk_load_checker_method!(glGenProgramPipelines, GenProgramPipelines);
  mk_load_checker_method!(glGenQueries, GenQueries);
  mk_load_checker_method!(glGenRenderbuffers, GenRenderbuffers);
  mk_load_checker_method!(glGenSamplers, GenSamplers);
  mk_load_checker_method!(glGenTextures, GenTextures);
  mk_load_checker_method!(glGenTransformFeedbacks, GenTransformFeedbacks);
  mk_load_checker_method!(glGenVertexArrays, GenVertexArrays);
  mk_load_checker_method!(glGenerateMipmap, GenerateMipmap);
  mk_load_checker_method!(glGenerateTextureMipmap, GenerateTextureMipmap);
  mk_load_checker_method!(glGetActiveAtomicCounterBufferiv, GetActiveAtomicCounterBufferiv);
  mk_load_checker_method!(glGetActiveAttrib, GetActiveAttrib);
  mk_load_checker_method!(glGetActiveSubroutineName, GetActiveSubroutineName);
  mk_load_checker_method!(glGetActiveSubroutineUniformName, GetActiveSubroutineUniformName);
  mk_load_checker_method!(glGetActiveSubroutineUniformiv, GetActiveSubroutineUniformiv);
  mk_load_checker_method!(glGetActiveUniform, GetActiveUniform);
  mk_load_checker_method!(glGetActiveUniformBlockName, GetActiveUniformBlockName);
  mk_load_checker_method!(glGetActiveUniformBlockiv, GetActiveUniformBlockiv);
  mk_load_checker_method!(glGetActiveUniformName, GetActiveUniformName);
  mk_load_checker_method!(glGetActiveUniformsiv, GetActiveUniformsiv);
  mk_load_checker_method!(glGetAttachedShaders, GetAttachedShaders);
  mk_load_checker_method!(glGetAttribLocation, GetAttribLocation);
  mk_load_checker_method!(glGetBooleani_v, GetBooleani_v);
  mk_load_checker_method!(glGetBooleanv, GetBooleanv);
  mk_load_checker_method!(glGetBufferParameteri64v, GetBufferParameteri64v);
  mk_load_checker_method!(glGetBufferParameteriv, GetBufferParameteriv);
  mk_load_checker_method!(glGetBufferPointerv, GetBufferPointerv);
  mk_load_checker_method!(glGetBufferSubData, GetBufferSubData);
  mk_load_checker_method!(glGetCompressedTexImage, GetCompressedTexImage);
  mk_load_checker_method!(glGetCompressedTextureImage, GetCompressedTextureImage);
  mk_load_checker_method!(glGetCompressedTextureSubImage, GetCompressedTextureSubImage);
  mk_load_checker_method!(glGetDebugMessageLog, GetDebugMessageLog);
  mk_load_checker_method!(glGetDebugMessageLogKHR, GetDebugMessageLogKHR);
  mk_load_checker_method!(glGetDoublei_v, GetDoublei_v);
  mk_load_checker_method!(glGetDoublev, GetDoublev);
  mk_load_checker_method!(glGetError, GetError);
  mk_load_checker_method!(glGetFloati_v, GetFloati_v);
  mk_load_checker_method!(glGetFloatv, GetFloatv);
  mk_load_checker_method!(glGetFragDataIndex, GetFragDataIndex);
  mk_load_checker_method!(glGetFragDataLocation, GetFragDataLocation);
  mk_load_checker_method!(
    glGetFramebufferAttachmentParameteriv,
    GetFramebufferAttachmentParameteriv
  );
  mk_load_checker_method!(glGetFramebufferParameteriv, GetFramebufferParameteriv);
  mk_load_checker_method!(glGetGraphicsResetStatus, GetGraphicsResetStatus);
  mk_load_checker_method!(glGetInteger64i_v, GetInteger64i_v);
  mk_load_checker_method!(glGetInteger64v, GetInteger64v);
  mk_load_checker_method!(glGetIntegeri_v, GetIntegeri_v);
  mk_load_checker_method!(glGetIntegerv, GetIntegerv);
  mk_load_checker_method!(glGetInternalformati64v, GetInternalformati64v);
  mk_load_checker_method!(glGetInternalformativ, GetInternalformativ);
  mk_load_checker_method!(glGetMultisamplefv, GetMultisamplefv);
  mk_load_checker_method!(glGetNamedBufferParameteri64v, GetNamedBufferParameteri64v);
  mk_load_checker_method!(glGetNamedBufferParameteriv, GetNamedBufferParameteriv);
  mk_load_checker_method!(glGetNamedBufferPointerv, GetNamedBufferPointerv);
  mk_load_checker_method!(glGetNamedBufferSubData, GetNamedBufferSubData);
  mk_load_checker_method!(
    glGetNamedFramebufferAttachmentParameteriv,
    GetNamedFramebufferAttachmentParameteriv
  );
  mk_load_checker_method!(glGetNamedFramebufferParameteriv, GetNamedFramebufferParameteriv);
  mk_load_checker_method!(glGetNamedRenderbufferParameteriv, GetNamedRenderbufferParameteriv);
  mk_load_checker_method!(glGetObjectLabel, GetObjectLabel);
  mk_load_checker_method!(glGetObjectLabelKHR, GetObjectLabelKHR);
  mk_load_checker_method!(glGetObjectPtrLabel, GetObjectPtrLabel);
  mk_load_checker_method!(glGetObjectPtrLabelKHR, GetObjectPtrLabelKHR);
  mk_load_checker_method!(glGetPointerv, GetPointerv);
  mk_load_checker_method!(glGetPointervKHR, GetPointervKHR);
  mk_load_checker_method!(glGetProgramBinary, GetProgramBinary);
  mk_load_checker_method!(glGetProgramInfoLog, GetProgramInfoLog);
  mk_load_checker_method!(glGetProgramInterfaceiv, GetProgramInterfaceiv);
  mk_load_checker_method!(glGetProgramPipelineInfoLog, GetProgramPipelineInfoLog);
  mk_load_checker_method!(glGetProgramPipelineiv, GetProgramPipelineiv);
  mk_load_checker_method!(glGetProgramResourceIndex, GetProgramResourceIndex);
  mk_load_checker_method!(glGetProgramResourceLocation, GetProgramResourceLocation);
  mk_load_checker_method!(glGetProgramResourceLocationIndex, GetProgramResourceLocationIndex);
  mk_load_checker_method!(glGetProgramResourceName, GetProgramResourceName);
  mk_load_checker_method!(glGetProgramResourceiv, GetProgramResourceiv);
  mk_load_checker_method!(glGetProgramStageiv, GetProgramStageiv);
  mk_load_checker_method!(glGetProgramiv, GetProgramiv);
  mk_load_checker_method!(glGetQueryBufferObjecti64v, GetQueryBufferObjecti64v);
  mk_load_checker_method!(glGetQueryBufferObjectiv, GetQueryBufferObjectiv);
  mk_load_checker_method!(glGetQueryBufferObjectui64v, GetQueryBufferObjectui64v);
  mk_load_checker_method!(glGetQueryBufferObjectuiv, GetQueryBufferObjectuiv);
  mk_load_checker_method!(glGetQueryIndexediv, GetQueryIndexediv);
  mk_load_checker_method!(glGetQueryObjecti64v, GetQueryObjecti64v);
  mk_load_checker_method!(glGetQueryObjectiv, GetQueryObjectiv);
  mk_load_checker_method!(glGetQueryObjectui64v, GetQueryObjectui64v);
  mk_load_checker_method!(glGetQueryObjectuiv, GetQueryObjectuiv);
  mk_load_checker_method!(glGetQueryiv, GetQueryiv);
  mk_load_checker_method!(glGetRenderbufferParameteriv, GetRenderbufferParameteriv);
  mk_load_checker_method!(glGetSamplerParameterIiv, GetSamplerParameterIiv);
  mk_load_checker_method!(glGetSamplerParameterIuiv, GetSamplerParameterIuiv);
  mk_load_checker_method!(glGetSamplerParameterfv, GetSamplerParameterfv);
  mk_load_checker_method!(glGetSamplerParameteriv, GetSamplerParameteriv);
  mk_load_checker_method!(glGetShaderInfoLog, GetShaderInfoLog);
  mk_load_checker_method!(glGetShaderPrecisionFormat, GetShaderPrecisionFormat);
  mk_load_checker_method!(glGetShaderSource, GetShaderSource);
  mk_load_checker_method!(glGetShaderiv, GetShaderiv);
  mk_load_checker_method!(glGetString, GetString);
  mk_load_checker_method!(glGetStringi, GetStringi);
  mk_load_checker_method!(glGetSubroutineIndex, GetSubroutineIndex);
  mk_load_checker_method!(glGetSubroutineUniformLocation, GetSubroutineUniformLocation);
  mk_load_checker_method!(glGetSynciv, GetSynciv);
  mk_load_checker_method!(glGetTexImage, GetTexImage);
  mk_load_checker_method!(glGetTexLevelParameterfv, GetTexLevelParameterfv);
  mk_load_checker_method!(glGetTexLevelParameteriv, GetTexLevelParameteriv);
  mk_load_checker_method!(glGetTexParameterIiv, GetTexParameterIiv);
  mk_load_checker_method!(glGetTexParameterIuiv, GetTexParameterIuiv);
  mk_load_checker_method!(glGetTexParameterfv, GetTexParameterfv);
  mk_load_checker_method!(glGetTexParameteriv, GetTexParameteriv);
  mk_load_checker_method!(glGetTextureImage, GetTextureImage);
  mk_load_checker_method!(glGetTextureLevelParameterfv, GetTextureLevelParameterfv);
  mk_load_checker_method!(glGetTextureLevelParameteriv, GetTextureLevelParameteriv);
  mk_load_checker_method!(glGetTextureParameterIiv, GetTextureParameterIiv);
  mk_load_checker_method!(glGetTextureParameterIuiv, GetTextureParameterIuiv);
  mk_load_checker_method!(glGetTextureParameterfv, GetTextureParameterfv);
  mk_load_checker_method!(glGetTextureParameteriv, GetTextureParameteriv);
  mk_load_checker_method!(glGetTextureSubImage, GetTextureSubImage);
  mk_load_checker_method!(glGetTransformFeedbackVarying, GetTransformFeedbackVarying);
  mk_load_checker_method!(glGetTransformFeedbacki64_v, GetTransformFeedbacki64_v);
  mk_load_checker_method!(glGetTransformFeedbacki_v, GetTransformFeedbacki_v);
  mk_load_checker_method!(glGetTransformFeedbackiv, GetTransformFeedbackiv);
  mk_load_checker_method!(glGetUniformBlockIndex, GetUniformBlockIndex);
  mk_load_checker_method!(glGetUniformIndices, GetUniformIndices);
  mk_load_checker_method!(glGetUniformLocation, GetUniformLocation);
  mk_load_checker_method!(glGetUniformSubroutineuiv, GetUniformSubroutineuiv);
  mk_load_checker_method!(glGetUniformdv, GetUniformdv);
  mk_load_checker_method!(glGetUniformfv, GetUniformfv);
  mk_load_checker_method!(glGetUniformiv, GetUniformiv);
  mk_load_checker_method!(glGetUniformuiv, GetUniformuiv);
  mk_load_checker_method!(glGetVertexArrayIndexed64iv, GetVertexArrayIndexed64iv);
  mk_load_checker_method!(glGetVertexArrayIndexediv, GetVertexArrayIndexediv);
  mk_load_checker_method!(glGetVertexArrayiv, GetVertexArrayiv);
  mk_load_checker_method!(glGetVertexAttribIiv, GetVertexAttribIiv);
  mk_load_checker_method!(glGetVertexAttribIuiv, GetVertexAttribIuiv);
  mk_load_checker_method!(glGetVertexAttribLdv, GetVertexAttribLdv);
  mk_load_checker_method!(glGetVertexAttribPointerv, GetVertexAttribPointerv);
  mk_load_checker_method!(glGetVertexAttribdv, GetVertexAttribdv);
  mk_load_checker_method!(glGetVertexAttribfv, GetVertexAttribfv);
  mk_load_checker_method!(glGetVertexAttribiv, GetVertexAttribiv);
  mk_load_checker_method!(glGetnColorTable, GetnColorTable);
  mk_load_checker_method!(glGetnCompressedTexImage, GetnCompressedTexImage);
  mk_load_checker_method!(glGetnConvolutionFilter, GetnConvolutionFilter);
  mk_load_checker_method!(glGetnHistogram, GetnHistogram);
  mk_load_checker_method!(glGetnMapdv, GetnMapdv);
  mk_load_checker_method!(glGetnMapfv, GetnMapfv);
  mk_load_checker_method!(glGetnMapiv, GetnMapiv);
  mk_load_checker_method!(glGetnMinmax, GetnMinmax);
  mk_load_checker_method!(glGetnPixelMapfv, GetnPixelMapfv);
  mk_load_checker_method!(glGetnPixelMapuiv, GetnPixelMapuiv);
  mk_load_checker_method!(glGetnPixelMapusv, GetnPixelMapusv);
  mk_load_checker_method!(glGetnPolygonStipple, GetnPolygonStipple);
  mk_load_checker_method!(glGetnSeparableFilter, GetnSeparableFilter);
  mk_load_checker_method!(glGetnTexImage, GetnTexImage);
  mk_load_checker_method!(glGetnUniformdv, GetnUniformdv);
  mk_load_checker_method!(glGetnUniformfv, GetnUniformfv);
  mk_load_checker_method!(glGetnUniformiv, GetnUniformiv);
  mk_load_checker_method!(glGetnUniformuiv, GetnUniformuiv);
  mk_load_checker_method!(glHint, Hint);
  mk_load_checker_method!(glInvalidateBufferData, InvalidateBufferData);
  mk_load_checker_method!(glInvalidateBufferSubData, InvalidateBufferSubData);
  mk_load_checker_method!(glInvalidateFramebuffer, InvalidateFramebuffer);
  mk_load_checker_method!(glInvalidateNamedFramebufferData, InvalidateNamedFramebufferData);
  mk_load_checker_method!(glInvalidateNamedFramebufferSubData, InvalidateNamedFramebufferSubData);
  mk_load_checker_method!(glInvalidateSubFramebuffer, InvalidateSubFramebuffer);
  mk_load_checker_method!(glInvalidateTexImage, InvalidateTexImage);
  mk_load_checker_method!(glInvalidateTexSubImage, InvalidateTexSubImage);
  mk_load_checker_method!(glIsBuffer, IsBuffer);
  mk_load_checker_method!(glIsEnabled, IsEnabled);
  mk_load_checker_method!(glIsEnabledi, IsEnabledi);
  mk_load_checker_method!(glIsFramebuffer, IsFramebuffer);
  mk_load_checker_method!(glIsProgram, IsProgram);
  mk_load_checker_method!(glIsProgramPipeline, IsProgramPipeline);
  mk_load_checker_method!(glIsQuery, IsQuery);
  mk_load_checker_method!(glIsRenderbuffer, IsRenderbuffer);
  mk_load_checker_method!(glIsSampler, IsSampler);
  mk_load_checker_method!(glIsShader, IsShader);
  mk_load_checker_method!(glIsSync, IsSync);
  mk_load_checker_method!(glIsTexture, IsTexture);
  mk_load_checker_method!(glIsTransformFeedback, IsTransformFeedback);
  mk_load_checker_method!(glIsVertexArray, IsVertexArray);
  mk_load_checker_method!(glLineWidth, LineWidth);
  mk_load_checker_method!(glLinkProgram, LinkProgram);
  mk_load_checker_method!(glLogicOp, LogicOp);
  mk_load_checker_method!(glMapBuffer, MapBuffer);
  mk_load_checker_method!(glMapBufferRange, MapBufferRange);
  mk_load_checker_method!(glMapNamedBuffer, MapNamedBuffer);
  mk_load_checker_method!(glMapNamedBufferRange, MapNamedBufferRange);
  mk_load_checker_method!(glMemoryBarrier, MemoryBarrier);
  mk_load_checker_method!(glMemoryBarrierByRegion, MemoryBarrierByRegion);
  mk_load_checker_method!(glMinSampleShading, MinSampleShading);
  mk_load_checker_method!(glMultiDrawArrays, MultiDrawArrays);
  mk_load_checker_method!(glMultiDrawArraysIndirect, MultiDrawArraysIndirect);
  mk_load_checker_method!(glMultiDrawArraysIndirectCount, MultiDrawArraysIndirectCount);
  mk_load_checker_method!(glMultiDrawElements, MultiDrawElements);
  mk_load_checker_method!(glMultiDrawElementsBaseVertex, MultiDrawElementsBaseVertex);
  mk_load_checker_method!(glMultiDrawElementsIndirect, MultiDrawElementsIndirect);
  mk_load_checker_method!(glMultiDrawElementsIndirectCount, MultiDrawElementsIndirectCount);
  mk_load_checker_method!(glMultiTexCoordP1ui, MultiTexCoordP1ui);
  mk_load_checker_method!(glMultiTexCoordP1uiv, MultiTexCoordP1uiv);
  mk_load_checker_method!(glMultiTexCoordP2ui, MultiTexCoordP2ui);
  mk_load_checker_method!(glMultiTexCoordP2uiv, MultiTexCoordP2uiv);
  mk_load_checker_method!(glMultiTexCoordP3ui, MultiTexCoordP3ui);
  mk_load_checker_method!(glMultiTexCoordP3uiv, MultiTexCoordP3uiv);
  mk_load_checker_method!(glMultiTexCoordP4ui, MultiTexCoordP4ui);
  mk_load_checker_method!(glMultiTexCoordP4uiv, MultiTexCoordP4uiv);
  mk_load_checker_method!(glNamedBufferData, NamedBufferData);
  mk_load_checker_method!(glNamedBufferStorage, NamedBufferStorage);
  mk_load_checker_method!(glNamedBufferSubData, NamedBufferSubData);
  mk_load_checker_method!(glNamedFramebufferDrawBuffer, NamedFramebufferDrawBuffer);
  mk_load_checker_method!(glNamedFramebufferDrawBuffers, NamedFramebufferDrawBuffers);
  mk_load_checker_method!(glNamedFramebufferParameteri, NamedFramebufferParameteri);
  mk_load_checker_method!(glNamedFramebufferReadBuffer, NamedFramebufferReadBuffer);
  mk_load_checker_method!(glNamedFramebufferRenderbuffer, NamedFramebufferRenderbuffer);
  mk_load_checker_method!(glNamedFramebufferTexture, NamedFramebufferTexture);
  mk_load_checker_method!(glNamedFramebufferTextureLayer, NamedFramebufferTextureLayer);
  mk_load_checker_method!(glNamedRenderbufferStorage, NamedRenderbufferStorage);
  mk_load_checker_method!(
    glNamedRenderbufferStorageMultisample,
    NamedRenderbufferStorageMultisample
  );
  mk_load_checker_method!(glNormalP3ui, NormalP3ui);
  mk_load_checker_method!(glNormalP3uiv, NormalP3uiv);
  mk_load_checker_method!(glObjectLabel, ObjectLabel);
  mk_load_checker_method!(glObjectLabelKHR, ObjectLabelKHR);
  mk_load_checker_method!(glObjectPtrLabel, ObjectPtrLabel);
  mk_load_checker_method!(glObjectPtrLabelKHR, ObjectPtrLabelKHR);
  mk_load_checker_method!(glPatchParameterfv, PatchParameterfv);
  mk_load_checker_method!(glPatchParameteri, PatchParameteri);
  mk_load_checker_method!(glPauseTransformFeedback, PauseTransformFeedback);
  mk_load_checker_method!(glPixelStoref, PixelStoref);
  mk_load_checker_method!(glPixelStorei, PixelStorei);
  mk_load_checker_method!(glPointParameterf, PointParameterf);
  mk_load_checker_method!(glPointParameterfv, PointParameterfv);
  mk_load_checker_method!(glPointParameteri, PointParameteri);
  mk_load_checker_method!(glPointParameteriv, PointParameteriv);
  mk_load_checker_method!(glPointSize, PointSize);
  mk_load_checker_method!(glPolygonMode, PolygonMode);
  mk_load_checker_method!(glPolygonOffset, PolygonOffset);
  mk_load_checker_method!(glPolygonOffsetClamp, PolygonOffsetClamp);
  mk_load_checker_method!(glPopDebugGroup, PopDebugGroup);
  mk_load_checker_method!(glPopDebugGroupKHR, PopDebugGroupKHR);
  mk_load_checker_method!(glPrimitiveBoundingBox, PrimitiveBoundingBox);
  mk_load_checker_method!(glPrimitiveRestartIndex, PrimitiveRestartIndex);
  mk_load_checker_method!(glProgramBinary, ProgramBinary);
  mk_load_checker_method!(glProgramParameteri, ProgramParameteri);
  mk_load_checker_method!(glProgramUniform1d, ProgramUniform1d);
  mk_load_checker_method!(glProgramUniform1dv, ProgramUniform1dv);
  mk_load_checker_method!(glProgramUniform1f, ProgramUniform1f);
  mk_load_checker_method!(glProgramUniform1fv, ProgramUniform1fv);
  mk_load_checker_method!(glProgramUniform1i, ProgramUniform1i);
  mk_load_checker_method!(glProgramUniform1iv, ProgramUniform1iv);
  mk_load_checker_method!(glProgramUniform1ui, ProgramUniform1ui);
  mk_load_checker_method!(glProgramUniform1uiv, ProgramUniform1uiv);
  mk_load_checker_method!(glProgramUniform2d, ProgramUniform2d);
  mk_load_checker_method!(glProgramUniform2dv, ProgramUniform2dv);
  mk_load_checker_method!(glProgramUniform2f, ProgramUniform2f);
  mk_load_checker_method!(glProgramUniform2fv, ProgramUniform2fv);
  mk_load_checker_method!(glProgramUniform2i, ProgramUniform2i);
  mk_load_checker_method!(glProgramUniform2iv, ProgramUniform2iv);
  mk_load_checker_method!(glProgramUniform2ui, ProgramUniform2ui);
  mk_load_checker_method!(glProgramUniform2uiv, ProgramUniform2uiv);
  mk_load_checker_method!(glProgramUniform3d, ProgramUniform3d);
  mk_load_checker_method!(glProgramUniform3dv, ProgramUniform3dv);
  mk_load_checker_method!(glProgramUniform3f, ProgramUniform3f);
  mk_load_checker_method!(glProgramUniform3fv, ProgramUniform3fv);
  mk_load_checker_method!(glProgramUniform3i, ProgramUniform3i);
  mk_load_checker_method!(glProgramUniform3iv, ProgramUniform3iv);
  mk_load_checker_method!(glProgramUniform3ui, ProgramUniform3ui);
  mk_load_checker_method!(glProgramUniform3uiv, ProgramUniform3uiv);
  mk_load_checker_method!(glProgramUniform4d, ProgramUniform4d);
  mk_load_checker_method!(glProgramUniform4dv, ProgramUniform4dv);
  mk_load_checker_method!(glProgramUniform4f, ProgramUniform4f);
  mk_load_checker_method!(glProgramUniform4fv, ProgramUniform4fv);
  mk_load_checker_method!(glProgramUniform4i, ProgramUniform4i);
  mk_load_checker_method!(glProgramUniform4iv, ProgramUniform4iv);
  mk_load_checker_method!(glProgramUniform4ui, ProgramUniform4ui);
  mk_load_checker_method!(glProgramUniform4uiv, ProgramUniform4uiv);
  mk_load_checker_method!(glProgramUniformMatrix2dv, ProgramUniformMatrix2dv);
  mk_load_checker_method!(glProgramUniformMatrix2fv, ProgramUniformMatrix2fv);
  mk_load_checker_method!(glProgramUniformMatrix2x3dv, ProgramUniformMatrix2x3dv);
  mk_load_checker_method!(glProgramUniformMatrix2x3fv, ProgramUniformMatrix2x3fv);
  mk_load_checker_method!(glProgramUniformMatrix2x4dv, ProgramUniformMatrix2x4dv);
  mk_load_checker_method!(glProgramUniformMatrix2x4fv, ProgramUniformMatrix2x4fv);
  mk_load_checker_method!(glProgramUniformMatrix3dv, ProgramUniformMatrix3dv);
  mk_load_checker_method!(glProgramUniformMatrix3fv, ProgramUniformMatrix3fv);
  mk_load_checker_method!(glProgramUniformMatrix3x2dv, ProgramUniformMatrix3x2dv);
  mk_load_checker_method!(glProgramUniformMatrix3x2fv, ProgramUniformMatrix3x2fv);
  mk_load_checker_method!(glProgramUniformMatrix3x4dv, ProgramUniformMatrix3x4dv);
  mk_load_checker_method!(glProgramUniformMatrix3x4fv, ProgramUniformMatrix3x4fv);
  mk_load_checker_method!(glProgramUniformMatrix4dv, ProgramUniformMatrix4dv);
  mk_load_checker_method!(glProgramUniformMatrix4fv, ProgramUniformMatrix4fv);
  mk_load_checker_method!(glProgramUniformMatrix4x2dv, ProgramUniformMatrix4x2dv);
  mk_load_checker_method!(glProgramUniformMatrix4x2fv, ProgramUniformMatrix4x2fv);
  mk_load_checker_method!(glProgramUniformMatrix4x3dv, ProgramUniformMatrix4x3dv);
  mk_load_checker_method!(glProgramUniformMatrix4x3fv, ProgramUniformMatrix4x3fv);
  mk_load_checker_method!(glProvokingVertex, ProvokingVertex);
  mk_load_checker_method!(glPushDebugGroup, PushDebugGroup);
  mk_load_checker_method!(glPushDebugGroupKHR, PushDebugGroupKHR);
  mk_load_checker_method!(glQueryCounter, QueryCounter);
  mk_load_checker_method!(glReadBuffer, ReadBuffer);
  mk_load_checker_method!(glReadPixels, ReadPixels);
  mk_load_checker_method!(glReadnPixels, ReadnPixels);
  mk_load_checker_method!(glReleaseShaderCompiler, ReleaseShaderCompiler);
  mk_load_checker_method!(glRenderbufferStorage, RenderbufferStorage);
  mk_load_checker_method!(glRenderbufferStorageMultisample, RenderbufferStorageMultisample);
  mk_load_checker_method!(glResumeTransformFeedback, ResumeTransformFeedback);
  mk_load_checker_method!(glSampleCoverage, SampleCoverage);
  mk_load_checker_method!(glSampleMaski, SampleMaski);
  mk_load_checker_method!(glSamplerParameterIiv, SamplerParameterIiv);
  mk_load_checker_method!(glSamplerParameterIuiv, SamplerParameterIuiv);
  mk_load_checker_method!(glSamplerParameterf, SamplerParameterf);
  mk_load_checker_method!(glSamplerParameterfv, SamplerParameterfv);
  mk_load_checker_method!(glSamplerParameteri, SamplerParameteri);
  mk_load_checker_method!(glSamplerParameteriv, SamplerParameteriv);
  mk_load_checker_method!(glScissor, Scissor);
  mk_load_checker_method!(glScissorArrayv, ScissorArrayv);
  mk_load_checker_method!(glScissorIndexed, ScissorIndexed);
  mk_load_checker_method!(glScissorIndexedv, ScissorIndexedv);
  mk_load_checker_method!(glSecondaryColorP3ui, SecondaryColorP3ui);
  mk_load_checker_method!(glSecondaryColorP3uiv, SecondaryColorP3uiv);
  mk_load_checker_method!(glShaderBinary, ShaderBinary);
  mk_load_checker_method!(glShaderSource, ShaderSource);
  mk_load_checker_method!(glShaderStorageBlockBinding, ShaderStorageBlockBinding);
  mk_load_checker_method!(glSpecializeShader, SpecializeShader);
  mk_load_checker_method!(glStencilFunc, StencilFunc);
  mk_load_checker_method!(glStencilFuncSeparate, StencilFuncSeparate);
  mk_load_checker_method!(glStencilMask, StencilMask);
  mk_load_checker_method!(glStencilMaskSeparate, StencilMaskSeparate);
  mk_load_checker_method!(glStencilOp, StencilOp);
  mk_load_checker_method!(glStencilOpSeparate, StencilOpSeparate);
  mk_load_checker_method!(glTexBuffer, TexBuffer);
  mk_load_checker_method!(glTexBufferRange, TexBufferRange);
  mk_load_checker_method!(glTexCoordP1ui, TexCoordP1ui);
  mk_load_checker_method!(glTexCoordP1uiv, TexCoordP1uiv);
  mk_load_checker_method!(glTexCoordP2ui, TexCoordP2ui);
  mk_load_checker_method!(glTexCoordP2uiv, TexCoordP2uiv);
  mk_load_checker_method!(glTexCoordP3ui, TexCoordP3ui);
  mk_load_checker_method!(glTexCoordP3uiv, TexCoordP3uiv);
  mk_load_checker_method!(glTexCoordP4ui, TexCoordP4ui);
  mk_load_checker_method!(glTexCoordP4uiv, TexCoordP4uiv);
  mk_load_checker_method!(glTexImage1D, TexImage1D);
  mk_load_checker_method!(glTexImage2D, TexImage2D);
  mk_load_checker_method!(glTexImage2DMultisample, TexImage2DMultisample);
  mk_load_checker_method!(glTexImage3D, TexImage3D);
  mk_load_checker_method!(glTexImage3DMultisample, TexImage3DMultisample);
  mk_load_checker_method!(glTexParameterIiv, TexParameterIiv);
  mk_load_checker_method!(glTexParameterIuiv, TexParameterIuiv);
  mk_load_checker_method!(glTexParameterf, TexParameterf);
  mk_load_checker_method!(glTexParameterfv, TexParameterfv);
  mk_load_checker_method!(glTexParameteri, TexParameteri);
  mk_load_checker_method!(glTexParameteriv, TexParameteriv);
  mk_load_checker_method!(glTexStorage1D, TexStorage1D);
  mk_load_checker_method!(glTexStorage2D, TexStorage2D);
  mk_load_checker_method!(glTexStorage2DMultisample, TexStorage2DMultisample);
  mk_load_checker_method!(glTexStorage3D, TexStorage3D);
  mk_load_checker_method!(glTexStorage3DMultisample, TexStorage3DMultisample);
  mk_load_checker_method!(glTexSubImage1D, TexSubImage1D);
  mk_load_checker_method!(glTexSubImage2D, TexSubImage2D);
  mk_load_checker_method!(glTexSubImage3D, TexSubImage3D);
  mk_load_checker_method!(glTextureBarrier, TextureBarrier);
  mk_load_checker_method!(glTextureBuffer, TextureBuffer);
  mk_load_checker_method!(glTextureBufferRange, TextureBufferRange);
  mk_load_checker_method!(glTextureParameterIiv, TextureParameterIiv);
  mk_load_checker_method!(glTextureParameterIuiv, TextureParameterIuiv);
  mk_load_checker_method!(glTextureParameterf, TextureParameterf);
  mk_load_checker_method!(glTextureParameterfv, TextureParameterfv);
  mk_load_checker_method!(glTextureParameteri, TextureParameteri);
  mk_load_checker_method!(glTextureParameteriv, TextureParameteriv);
  mk_load_checker_method!(glTextureStorage1D, TextureStorage1D);
  mk_load_checker_method!(glTextureStorage2D, TextureStorage2D);
  mk_load_checker_method!(glTextureStorage2DMultisample, TextureStorage2DMultisample);
  mk_load_checker_method!(glTextureStorage3D, TextureStorage3D);
  mk_load_checker_method!(glTextureStorage3DMultisample, TextureStorage3DMultisample);
  mk_load_checker_method!(glTextureSubImage1D, TextureSubImage1D);
  mk_load_checker_method!(glTextureSubImage2D, TextureSubImage2D);
  mk_load_checker_method!(glTextureSubImage3D, TextureSubImage3D);
  mk_load_checker_method!(glTextureView, TextureView);
  mk_load_checker_method!(glTransformFeedbackBufferBase, TransformFeedbackBufferBase);
  mk_load_checker_method!(glTransformFeedbackBufferRange, TransformFeedbackBufferRange);
  mk_load_checker_method!(glTransformFeedbackVaryings, TransformFeedbackVaryings);
  mk_load_checker_method!(glUniform1d, Uniform1d);
  mk_load_checker_method!(glUniform1dv, Uniform1dv);
  mk_load_checker_method!(glUniform1f, Uniform1f);
  mk_load_checker_method!(glUniform1fv, Uniform1fv);
  mk_load_checker_method!(glUniform1i, Uniform1i);
  mk_load_checker_method!(glUniform1iv, Uniform1iv);
  mk_load_checker_method!(glUniform1ui, Uniform1ui);
  mk_load_checker_method!(glUniform1uiv, Uniform1uiv);
  mk_load_checker_method!(glUniform2d, Uniform2d);
  mk_load_checker_method!(glUniform2dv, Uniform2dv);
  mk_load_checker_method!(glUniform2f, Uniform2f);
  mk_load_checker_method!(glUniform2fv, Uniform2fv);
  mk_load_checker_method!(glUniform2i, Uniform2i);
  mk_load_checker_method!(glUniform2iv, Uniform2iv);
  mk_load_checker_method!(glUniform2ui, Uniform2ui);
  mk_load_checker_method!(glUniform2uiv, Uniform2uiv);
  mk_load_checker_method!(glUniform3d, Uniform3d);
  mk_load_checker_method!(glUniform3dv, Uniform3dv);
  mk_load_checker_method!(glUniform3f, Uniform3f);
  mk_load_checker_method!(glUniform3fv, Uniform3fv);
  mk_load_checker_method!(glUniform3i, Uniform3i);
  mk_load_checker_method!(glUniform3iv, Uniform3iv);
  mk_load_checker_method!(glUniform3ui, Uniform3ui);
  mk_load_checker_method!(glUniform3uiv, Uniform3uiv);
  mk_load_checker_method!(glUniform4d, Uniform4d);
  mk_load_checker_method!(glUniform4dv, Uniform4dv);
  mk_load_checker_method!(glUniform4f, Uniform4f);
  mk_load_checker_method!(glUniform4fv, Uniform4fv);
  mk_load_checker_method!(glUniform4i, Uniform4i);
  mk_load_checker_method!(glUniform4iv, Uniform4iv);
  mk_load_checker_method!(glUniform4ui, Uniform4ui);
  mk_load_checker_method!(glUniform4uiv, Uniform4uiv);
  mk_load_checker_method!(glUniformBlockBinding, UniformBlockBinding);
  mk_load_checker_method!(glUniformMatrix2dv, UniformMatrix2dv);
  mk_load_checker_method!(glUniformMatrix2fv, UniformMatrix2fv);
  mk_load_checker_method!(glUniformMatrix2x3dv, UniformMatrix2x3dv);
  mk_load_checker_method!(glUniformMatrix2x3fv, UniformMatrix2x3fv);
  mk_load_checker_method!(glUniformMatrix2x4dv, UniformMatrix2x4dv);
  mk_load_checker_method!(glUniformMatrix2x4fv, UniformMatrix2x4fv);
  mk_load_checker_method!(glUniformMatrix3dv, UniformMatrix3dv);
  mk_load_checker_method!(glUniformMatrix3fv, UniformMatrix3fv);
  mk_load_checker_method!(glUniformMatrix3x2dv, UniformMatrix3x2dv);
  mk_load_checker_method!(glUniformMatrix3x2fv, UniformMatrix3x2fv);
  mk_load_checker_method!(glUniformMatrix3x4dv, UniformMatrix3x4dv);
  mk_load_checker_method!(glUniformMatrix3x4fv, UniformMatrix3x4fv);
  mk_load_checker_method!(glUniformMatrix4dv, UniformMatrix4dv);
  mk_load_checker_method!(glUniformMatrix4fv, UniformMatrix4fv);
  mk_load_checker_method!(glUniformMatrix4x2dv, UniformMatrix4x2dv);
  mk_load_checker_method!(glUniformMatrix4x2fv, UniformMatrix4x2fv);
  mk_load_checker_method!(glUniformMatrix4x3dv, UniformMatrix4x3dv);
  mk_load_checker_method!(glUniformMatrix4x3fv, UniformMatrix4x3fv);
  mk_load_checker_method!(glUniformSubroutinesuiv, UniformSubroutinesuiv);
  mk_load_checker_method!(glUnmapBuffer, UnmapBuffer);
  mk_load_checker_method!(glUnmapNamedBuffer, UnmapNamedBuffer);
  mk_load_checker_method!(glUseProgram, UseProgram);
  mk_load_checker_method!(glUseProgramStages, UseProgramStages);
  mk_load_checker_method!(glValidateProgram, ValidateProgram);
  mk_load_checker_method!(glValidateProgramPipeline, ValidateProgramPipeline);
  mk_load_checker_method!(glVertexArrayAttribBinding, VertexArrayAttribBinding);
  mk_load_checker_method!(glVertexArrayAttribFormat, VertexArrayAttribFormat);
  mk_load_checker_method!(glVertexArrayAttribIFormat, VertexArrayAttribIFormat);
  mk_load_checker_method!(glVertexArrayAttribLFormat, VertexArrayAttribLFormat);
  mk_load_checker_method!(glVertexArrayBindingDivisor, VertexArrayBindingDivisor);
  mk_load_checker_method!(glVertexArrayElementBuffer, VertexArrayElementBuffer);
  mk_load_checker_method!(glVertexArrayVertexBuffer, VertexArrayVertexBuffer);
  mk_load_checker_method!(glVertexArrayVertexBuffers, VertexArrayVertexBuffers);
  mk_load_checker_method!(glVertexAttrib1d, VertexAttrib1d);
  mk_load_checker_method!(glVertexAttrib1dv, VertexAttrib1dv);
  mk_load_checker_method!(glVertexAttrib1f, VertexAttrib1f);
  mk_load_checker_method!(glVertexAttrib1fv, VertexAttrib1fv);
  mk_load_checker_method!(glVertexAttrib1s, VertexAttrib1s);
  mk_load_checker_method!(glVertexAttrib1sv, VertexAttrib1sv);
  mk_load_checker_method!(glVertexAttrib2d, VertexAttrib2d);
  mk_load_checker_method!(glVertexAttrib2dv, VertexAttrib2dv);
  mk_load_checker_method!(glVertexAttrib2f, VertexAttrib2f);
  mk_load_checker_method!(glVertexAttrib2fv, VertexAttrib2fv);
  mk_load_checker_method!(glVertexAttrib2s, VertexAttrib2s);
  mk_load_checker_method!(glVertexAttrib2sv, VertexAttrib2sv);
  mk_load_checker_method!(glVertexAttrib3d, VertexAttrib3d);
  mk_load_checker_method!(glVertexAttrib3dv, VertexAttrib3dv);
  mk_load_checker_method!(glVertexAttrib3f, VertexAttrib3f);
  mk_load_checker_method!(glVertexAttrib3fv, VertexAttrib3fv);
  mk_load_checker_method!(glVertexAttrib3s, VertexAttrib3s);
  mk_load_checker_method!(glVertexAttrib3sv, VertexAttrib3sv);
  mk_load_checker_method!(glVertexAttrib4Nbv, VertexAttrib4Nbv);
  mk_load_checker_method!(glVertexAttrib4Niv, VertexAttrib4Niv);
  mk_load_checker_method!(glVertexAttrib4Nsv, VertexAttrib4Nsv);
  mk_load_checker_method!(glVertexAttrib4Nub, VertexAttrib4Nub);
  mk_load_checker_method!(glVertexAttrib4Nubv, VertexAttrib4Nubv);
  mk_load_checker_method!(glVertexAttrib4Nuiv, VertexAttrib4Nuiv);
  mk_load_checker_method!(glVertexAttrib4Nusv, VertexAttrib4Nusv);
  mk_load_checker_method!(glVertexAttrib4bv, VertexAttrib4bv);
  mk_load_checker_method!(glVertexAttrib4d, VertexAttrib4d);
  mk_load_checker_method!(glVertexAttrib4dv, VertexAttrib4dv);
  mk_load_checker_method!(glVertexAttrib4f, VertexAttrib4f);
  mk_load_checker_method!(glVertexAttrib4fv, VertexAttrib4fv);
  mk_load_checker_method!(glVertexAttrib4iv, VertexAttrib4iv);
  mk_load_checker_method!(glVertexAttrib4s, VertexAttrib4s);
  mk_load_checker_method!(glVertexAttrib4sv, VertexAttrib4sv);
  mk_load_checker_method!(glVertexAttrib4ubv, VertexAttrib4ubv);
  mk_load_checker_method!(glVertexAttrib4uiv, VertexAttrib4uiv);
  mk_load_checker_method!(glVertexAttrib4usv, VertexAttrib4usv);
  mk_load_checker_method!(glVertexAttribBinding, VertexAttribBinding);
  mk_load_checker_method!(glVertexAttribDivisor, VertexAttribDivisor);
  mk_load_checker_method!(glVertexAttribFormat, VertexAttribFormat);
  mk_load_checker_method!(glVertexAttribI1i, VertexAttribI1i);
  mk_load_checker_method!(glVertexAttribI1iv, VertexAttribI1iv);
  mk_load_checker_method!(glVertexAttribI1ui, VertexAttribI1ui);
  mk_load_checker_method!(glVertexAttribI1uiv, VertexAttribI1uiv);
  mk_load_checker_method!(glVertexAttribI2i, VertexAttribI2i);
  mk_load_checker_method!(glVertexAttribI2iv, VertexAttribI2iv);
  mk_load_checker_method!(glVertexAttribI2ui, VertexAttribI2ui);
  mk_load_checker_method!(glVertexAttribI2uiv, VertexAttribI2uiv);
  mk_load_checker_method!(glVertexAttribI3i, VertexAttribI3i);
  mk_load_checker_method!(glVertexAttribI3iv, VertexAttribI3iv);
  mk_load_checker_method!(glVertexAttribI3ui, VertexAttribI3ui);
  mk_load_checker_method!(glVertexAttribI3uiv, VertexAttribI3uiv);
  mk_load_checker_method!(glVertexAttribI4bv, VertexAttribI4bv);
  mk_load_checker_method!(glVertexAttribI4i, VertexAttribI4i);
  mk_load_checker_method!(glVertexAttribI4iv, VertexAttribI4iv);
  mk_load_checker_method!(glVertexAttribI4sv, VertexAttribI4sv);
  mk_load_checker_method!(glVertexAttribI4ubv, VertexAttribI4ubv);
  mk_load_checker_method!(glVertexAttribI4ui, VertexAttribI4ui);
  mk_load_checker_method!(glVertexAttribI4uiv, VertexAttribI4uiv);
  mk_load_checker_method!(glVertexAttribI4usv, VertexAttribI4usv);
  mk_load_checker_method!(glVertexAttribIFormat, VertexAttribIFormat);
  mk_load_checker_method!(glVertexAttribIPointer, VertexAttribIPointer);
  mk_load_checker_method!(glVertexAttribL1d, VertexAttribL1d);
  mk_load_checker_method!(glVertexAttribL1dv, VertexAttribL1dv);
  mk_load_checker_method!(glVertexAttribL2d, VertexAttribL2d);
  mk_load_checker_method!(glVertexAttribL2dv, VertexAttribL2dv);
  mk_load_checker_method!(glVertexAttribL3d, VertexAttribL3d);
  mk_load_checker_method!(glVertexAttribL3dv, VertexAttribL3dv);
  mk_load_checker_method!(glVertexAttribL4d, VertexAttribL4d);
  mk_load_checker_method!(glVertexAttribL4dv, VertexAttribL4dv);
  mk_load_checker_method!(glVertexAttribLFormat, VertexAttribLFormat);
  mk_load_checker_method!(glVertexAttribLPointer, VertexAttribLPointer);
  mk_load_checker_method!(glVertexAttribP1ui, VertexAttribP1ui);
  mk_load_checker_method!(glVertexAttribP1uiv, VertexAttribP1uiv);
  mk_load_checker_method!(glVertexAttribP2ui, VertexAttribP2ui);
  mk_load_checker_method!(glVertexAttribP2uiv, VertexAttribP2uiv);
  mk_load_checker_method!(glVertexAttribP3ui, VertexAttribP3ui);
  mk_load_checker_method!(glVertexAttribP3uiv, VertexAttribP3uiv);
  mk_load_checker_method!(glVertexAttribP4ui, VertexAttribP4ui);
  mk_load_checker_method!(glVertexAttribP4uiv, VertexAttribP4uiv);
  mk_load_checker_method!(glVertexAttribPointer, VertexAttribPointer);
  mk_load_checker_method!(glVertexBindingDivisor, VertexBindingDivisor);
  mk_load_checker_method!(glVertexP2ui, VertexP2ui);
  mk_load_checker_method!(glVertexP2uiv, VertexP2uiv);
  mk_load_checker_method!(glVertexP3ui, VertexP3ui);
  mk_load_checker_method!(glVertexP3uiv, VertexP3uiv);
  mk_load_checker_method!(glVertexP4ui, VertexP4ui);
  mk_load_checker_method!(glVertexP4uiv, VertexP4uiv);
  mk_load_checker_method!(glViewport, Viewport);
  mk_load_checker_method!(glViewportArrayv, ViewportArrayv);
  mk_load_checker_method!(glViewportIndexedf, ViewportIndexedf);
  mk_load_checker_method!(glViewportIndexedfv, ViewportIndexedfv);
  mk_load_checker_method!(glWaitSync, WaitSync);
}
