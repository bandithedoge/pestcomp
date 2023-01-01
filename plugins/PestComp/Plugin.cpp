#include "DistrhoPlugin.hpp"

START_NAMESPACE_DISTRHO

class PestComp : public Plugin {
  public:
    PestComp() : Plugin(paramCount, 0, 0){};

  protected:
    const char *getLabel() const override { return DISTRHO_PLUGIN_NAME; }

    const char *getDescription() const override {
        return "Really shitty digital compressor";
    }

    const char *getMaker() const override { return DISTRHO_PLUGIN_BRAND; }

    const char *getLicense() const override {
        return "Do What The Fuck You Want To Public License";
    }

    const char *getHomePage() const override { return DISTRHO_PLUGIN_URI; }

    uint32_t getVersion() const override { return d_version(2, 0, 0); }

    int64_t getUniqueId() const override {
        return d_cconst('p', 'r', 'P', 'C');
    }

    void initAudioPort(bool input, uint32_t index, AudioPort &port) override {
        port.groupId = kPortGroupStereo;
        Plugin::initAudioPort(input, index, port);
    };

    void initParameter(uint32_t index, Parameter &parameter) override {
        switch (index) {
        case paramEngage:
            parameter.hints = kParameterIsAutomatable | kParameterIsBoolean;
            parameter.name = "Engage";
            parameter.symbol = "engage";
        }
    };

    void initState(uint32_t, String &, String &){};

    float getParameterValue(uint32_t index) const override {
        switch (index) {
        case paramEngage:
            return fEngage;
        }

        return 0.0f;
    };

    void setParameterValue(uint32_t index, float value) override {
        switch (index) {
        case paramEngage:
            fEngage = value;
        }
    };

    void setState(const char *key, const char *);

    void run(const float **inputs, float **outputs, uint32_t frames) override {
        if (fEngage) {
            float *const outL = outputs[0];
            float *const outR = outputs[1];

            for (uint32_t i = 0; i < frames; ++i) {
                outL[i] = 0;
                outR[i] = 0;
            }
        } else {
            if (outputs[0] != inputs[0])
                std::memcpy(outputs[0], inputs[0], sizeof(float) * frames);
        }
    };

  private:
    volatile bool fEngage = false;
    DISTRHO_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(PestComp)
};

Plugin *createPlugin() { return new PestComp(); };

END_NAMESPACE_DISTRHO
