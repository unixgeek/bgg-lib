<?xml version="1.0" encoding="UTF-8"?>
<xsl:transform version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

    <!-- Identity transformation to copy all nodes -->
    <xsl:template match="@*|node()">
        <xsl:copy>
            <xsl:apply-templates select="@*|node()"/>
        </xsl:copy>
    </xsl:template>

    <!-- Template to match and exclude <poll-summary> elements -->
    <xsl:template match="poll-summary"/>

    <!-- Template to match and exclude <poll> elements where name attribute is not 'suggested_numplayers' -->
    <xsl:template match="poll[not(@name='suggested_numplayers')]"/>

</xsl:transform>
